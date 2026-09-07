use std::marker::PhantomData;

//
// Entity
//

// pub trait Entity {
//     type Model;

//     const TABLE: &'static str;

//     fn columns() -> &'static [Column];

//     fn find() -> Query<Self>
//     where
//         Self: Sized,
//     {
//         Query::new()
//     }
// }

pub trait Entity {
    type Model;

    const TABLE: &'static str;
    const COLUMNS: &'static [Column];

    fn columns() -> &'static [Column] {
        Self::COLUMNS
    }

    fn find() -> Query<Self>
    where
        Self: Sized,
    {
        Query::new()
    }
}

//
// Column metadata
//

#[derive(Debug, Clone, Copy)]
pub struct Column {
    name: &'static str,
}

impl Column {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

//
// Field
//

pub struct Field<E, T> {
    column: Column,
    _entity: PhantomData<E>,
    _type: PhantomData<T>,
}

impl<E, T> Field<E, T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            column: Column::new(name),
            _entity: PhantomData,
            _type: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.column.name()
    }
}

//
// Typed condition
//

pub struct Condition<E> {
    column: Column,
    value: BindValue,
    _entity: PhantomData<E>,
}

//
// Internal SQL bind representation
//

enum BindValue {
    String(String),
    I64(i64),
}

//
// String field expressions
//

impl<E> Field<E, String> {
    pub fn eq(&self, value: impl Into<String>) -> Condition<E> {
        Condition {
            column: self.column,
            value: BindValue::String(value.into()),
            _entity: PhantomData,
        }
    }
}

//
// Integer field expressions
//

impl<E> Field<E, i32> {
    pub fn eq(&self, value: i32) -> Condition<E> {
        Condition {
            column: self.column,
            value: BindValue::I64(value as i64),
            _entity: PhantomData,
        }
    }
}

//
// Select Statement
//
pub struct SelectStatement {
    columns: Vec<Column>,
    table: &'static str,
    where_clause: Option<ConditionValue>,
}

// pub struct Condition<E> { // ye upar define hy bhai
//     column: Column,
//     value: BindValue,
//     _entity: PhantomData<E>,
// }

struct ConditionValue {
    column: Column,
    value: BindValue,
}

impl<E> Condition<E> {
    fn into_ast(self) -> ConditionValue {
        ConditionValue {
            column: self.column,
            value: self.value,
        }
    }
}

//
// Query
//

// pub struct Query<E> {
//     condition: Option<Condition<E>>,
//     limit: Option<i64>,
//     _entity: PhantomData<E>,
// }

pub struct Query<E> {
    statement: SelectStatement,
    _entity: PhantomData<E>,
}

impl<E> Query<E>
where
    E: Entity,
{
    fn new() -> Self {
        Self {
            statement: SelectStatement {
                columns: E::columns().to_vec(),
                table: E::TABLE,
                where_clause: None,
            },
            _entity: PhantomData,
        }
    }

    pub fn where_(mut self, condition: Condition<E>) -> Self {
        self.statement.where_clause = Some(condition.into_ast());
        self
    }
}

//
// SQL generation
//

impl<E> Query<E>
where
    E: Entity,
{
    fn build_sql(&self) -> String {
        let columns = self
            .statement
            .columns
            .iter()
            .map(Column::name)
            .collect::<Vec<_>>()
            .join(", ");

        let mut sql = format!("SELECT {} FROM {}", columns, self.statement.table);

        if let Some(condition) = &self.statement.where_clause {
            sql.push_str(&format!(" WHERE {} = $1", condition.column.name()));
        }

        sql
    }
}

//
// PostgreSQL execution
//
// This is intentionally kept behind the query API for now.
//

impl<E> Query<E>
where
    E: Entity,
    for<'r> E::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    pub async fn all(self, db: &sqlx::PgPool) -> Result<Vec<E::Model>, sqlx::Error> {
        let sql = self.build_sql();

        let mut query = sqlx::query_as::<_, E::Model>(&sql);

        if let Some(condition) = self.statement.where_clause {
            match condition.value {
                BindValue::String(value) => {
                    query = query.bind(value);
                }

                BindValue::I64(value) => {
                    query = query.bind(value);
                }
            }
        }

        query.fetch_all(db).await
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;
    struct TestUser;
    #[derive(Debug)]
    struct TestUserModel;
    impl Entity for TestUser {
        type Model = TestUserModel;
        const TABLE: &'static str = "users";
        const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("name")];
    }
    impl TestUser {
        #[allow(non_upper_case_globals)]
        const id: Field<Self, i32> = Field::new("id");
        #[allow(non_upper_case_globals)]
        const name: Field<Self, String> = Field::new("name");
    }
    struct TestPost;
    #[derive(Debug)]
    struct TestPostModel;
    impl Entity for TestPost {
        type Model = TestPostModel;
        const TABLE: &'static str = "posts";
        const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("title")];
    }
    impl TestPost {
        #[allow(non_upper_case_globals)]
        #[allow(dead_code)]
        const id: Field<Self, i32> = Field::new("id");
        #[allow(non_upper_case_globals)]
        const title: Field<Self, String> = Field::new("title");
    }
    #[test]
    fn user_query_is_generic() {
        let query = TestUser::find().where_(TestUser::name.eq("Fakhir"));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE name = $1");
    }
    #[test]
    fn post_query_is_generic() {
        let query = TestPost::find().where_(TestPost::title.eq("Rust"));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, title FROM posts WHERE title = $1");
    }

    #[test]
    fn integer_field_is_typed() {
        let query = TestUser::find().where_(TestUser::id.eq(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id = $1");
    }

    #[test]
    fn condition_belongs_to_entity() {
        let condition = TestUser::name.eq("Fakhir");
        let query = TestUser::find().where_(condition);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE name = $1");
    }
}

use std::marker::PhantomData;

pub trait Entity {
    type Model;

    const TABLE: &'static str;

    fn columns() -> &'static [&'static str];

    fn find() -> Query<Self>
    where
        Self: Sized,
    {
        Query::new()
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl Entity for User {
    type Model = User;

    const TABLE: &'static str = "users";

    fn columns() -> &'static [&'static str] {
        &["id", "name"]
    }
}

pub struct Field<T> {
    name: &'static str,
    _marker: PhantomData<T>,
}

impl<T> Field<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }
}

impl User {
    #[allow(non_upper_case_globals)]
    pub const id: Field<i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const name: Field<String> = Field::new("name");
}

pub enum Value {
    String(String),
    I64(i64),
}

pub struct Condition {
    column: &'static str,
    value: Value,
}

impl Field<String> {
    pub fn eq(&self, value: impl Into<String>) -> Condition {
        Condition {
            column: self.name,
            value: Value::String(value.into()),
        }
    }
}

pub struct Query<E> {
    condition: Option<Condition>,
    limit: Option<i64>,
    _entity: PhantomData<E>,
}

impl<E> Query<E> {
    fn new() -> Self {
        Self {
            condition: None,
            limit: None,
            _entity: PhantomData,
        }
    }

    pub fn where_(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn take(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl<E> Query<E>
where
    E: Entity,
{
    fn build_sql(&self) -> String {
        let columns = E::columns().join(", ");

        let mut sql = format!("SELECT {} FROM {}", columns, E::TABLE);

        if let Some(condition) = &self.condition {
            sql.push_str(&format!(" WHERE {} = $1", condition.column));
        }

        if self.limit.is_some() {
            let index = if self.condition.is_some() { 2 } else { 1 };

            sql.push_str(&format!(" LIMIT ${index}"));
        }

        sql
    }
}

impl<E> Query<E>
where
    E: Entity,
    for<'r> E::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    pub async fn all(
        self,
        db: &sqlx::PgPool,
    ) -> Result<Vec<E::Model>, sqlx::Error> {
        let sql = self.build_sql();

        let mut query = sqlx::query_as::<_, E::Model>(&sql);

        if let Some(condition) = self.condition {
            match condition.value {
                Value::String(value) => {
                    query = query.bind(value);
                }

                Value::I64(value) => {
                    query = query.bind(value);
                }
            }
        }

        if let Some(limit) = self.limit {
            query = query.bind(limit);
        }

        query.fetch_all(db).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_user_query() {
        let query = User::find()
            .where_(User::name.eq("Fakhir"))
            .take(20);

        let sql = query.build_sql();

        assert_eq!(
            sql,
            "SELECT id, name FROM users WHERE name = $1 LIMIT $2"
        );
    }
}
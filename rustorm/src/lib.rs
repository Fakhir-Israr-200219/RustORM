use std::marker::PhantomData;

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

    pub fn asc(&self) -> OrderBy {
        OrderBy {
            column: self.column,
            direction: OrderDirection::Asc,
        }
    }

    pub fn desc(&self) -> OrderBy {
        OrderBy {
            column: self.column,
            direction: OrderDirection::Desc,
        }
    }
}

//
// Typed condition
//

pub struct Condition<E> {
    expression: Expression,
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
    fn compare(&self, operator: BinaryOperator, value: impl Into<String>) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(Expression::Column(self.column)),
                operator,
                right: Box::new(Expression::Value(BindValue::String(value.into()))),
            },
            _entity: PhantomData,
        }
    }

    pub fn eq(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Eq, value)
    }

    pub fn not_eq(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::NotEq, value)
    }

    pub fn gt(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Gt, value)
    }

    pub fn gte(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Gte, value)
    }

    pub fn lt(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Lt, value)
    }

    pub fn lte(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Lte, value)
    }
}

//
// Integer field expressions
//

impl<E> Field<E, i32> {
    fn compare(&self, operator: BinaryOperator, value: i32) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(Expression::Column(self.column)),
                operator,
                right: Box::new(Expression::Value(BindValue::I64(value as i64))),
            },
            _entity: PhantomData,
        }
    }

    pub fn eq(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Eq, value)
    }

    pub fn not_eq(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::NotEq, value)
    }

    pub fn gt(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Gt, value)
    }

    pub fn gte(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Gte, value)
    }

    pub fn lt(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Lt, value)
    }

    pub fn lte(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Lte, value)
    }
}

//
// Select Statement
//
pub struct SelectStatement {
    columns: Vec<Column>,
    table: &'static str,
    where_clause: Option<Expression>,
    order_by: Option<OrderBy>,
    limit: Option<u64>,
    offset: Option<u64>,
    distinct: bool,
    group_by: Vec<Column>,
}
enum OrderDirection {
    Asc,
    Desc,
}

pub struct OrderBy {
    column: Column,
    direction: OrderDirection,
}
enum BinaryOperator {
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    And,
    Or,
}
enum Expression {
    Column(Column),
    Value(BindValue),
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

impl<E> Condition<E> {
    pub fn and(self, other: Condition<E>) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(self.expression),
                operator: BinaryOperator::And,
                right: Box::new(other.expression),
            },
            _entity: PhantomData,
        }
    }

    pub fn or(self, other: Condition<E>) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(self.expression),
                operator: BinaryOperator::Or,
                right: Box::new(other.expression),
            },
            _entity: PhantomData,
        }
    }

    fn into_ast(self) -> Expression {
        self.expression
    }
}

//
// Query
//

pub struct Query<E> {
    statement: SelectStatement,
    _entity: PhantomData<E>,
}
fn collect_bind_values(expression: &Expression, values: &mut Vec<BindValue>) {
    match expression {
        Expression::Column(_) => {}

        Expression::Value(value) => match value {
            BindValue::String(value) => {
                values.push(BindValue::String(value.clone()));
            }

            BindValue::I64(value) => {
                values.push(BindValue::I64(*value));
            }
        },

        Expression::Binary { left, right, .. } => {
            collect_bind_values(left, values);
            collect_bind_values(right, values);
        }
    }
}
fn compile_expression(expression: &Expression, next_placeholder: &mut usize) -> String {
    match expression {
        Expression::Column(column) => column.name().to_string(),

        Expression::Value(_) => {
            let placeholder = format!("${}", *next_placeholder);
            *next_placeholder += 1;
            placeholder
        }

        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left_sql = compile_expression(left, next_placeholder);

            let operator_sql = match operator {
                BinaryOperator::Eq => "=",
                BinaryOperator::NotEq => "<>",
                BinaryOperator::Gt => ">",
                BinaryOperator::Gte => ">=",
                BinaryOperator::Lt => "<",
                BinaryOperator::Lte => "<=",
                BinaryOperator::And => "AND",
                BinaryOperator::Or => "OR",
            };

            let right_sql = compile_expression(right, next_placeholder);

            format!("{} {} {}", left_sql, operator_sql, right_sql)
        }
    }
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
                order_by: None,
                limit: None,
                offset: None,
                distinct: false,
                group_by: Vec::new(),
            },
            _entity: PhantomData,
        }
    }

    pub fn where_(mut self, condition: Condition<E>) -> Self {
        self.statement.where_clause = Some(condition.into_ast());
        self
    }

    pub fn order_by(mut self, order: OrderBy) -> Self {
        self.statement.order_by = Some(order);
        self
    }

    pub fn take(mut self, limit: u64) -> Self {
        self.statement.limit = Some(limit);
        self
    }

    pub fn skip(mut self, offset: u64) -> Self {
        self.statement.offset = Some(offset);
        self
    }

    pub fn distinct(mut self) -> Self {
        self.statement.distinct = true;
        self
    }

    pub fn group_by(mut self, column: Column) -> Self {
        self.statement.group_by.push(column);
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

        let mut sql = if self.statement.distinct {
            format!("SELECT DISTINCT {} FROM {}", columns, self.statement.table)
        } else {
            format!("SELECT {} FROM {}", columns, self.statement.table)
        };

        let mut next_placeholder = 1;

        if let Some(condition) = &self.statement.where_clause {
            sql.push_str(" WHERE ");
            sql.push_str(&compile_expression(condition, &mut next_placeholder));
        }

        if !self.statement.group_by.is_empty() {
            sql.push_str(" GROUP BY ");

            let columns = self
                .statement
                .group_by
                .iter()
                .map(|column| column.name())
                .collect::<Vec<_>>()
                .join(", ");

            sql.push_str(&columns);
        }

        if let Some(order_by) = &self.statement.order_by {
            sql.push_str(" ORDER BY ");
            sql.push_str(order_by.column.name());

            match order_by.direction {
                OrderDirection::Asc => {
                    sql.push_str(" ASC");
                }

                OrderDirection::Desc => {
                    sql.push_str(" DESC");
                }
            }
        }

        if let Some(limit) = self.statement.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.statement.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        sql
    }
}

impl<E> Query<E>
where
    E: Entity,
    for<'r> E::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    pub async fn all(self, db: &sqlx::PgPool) -> Result<Vec<E::Model>, sqlx::Error> {
        let sql = self.build_sql();
        let mut query = sqlx::query_as::<_, E::Model>(&sql);

        if let Some(expression) = &self.statement.where_clause {
            let mut values = Vec::new();

            collect_bind_values(expression, &mut values);
            for value in values {
                match value {
                    BindValue::String(value) => {
                        query = query.bind(value);
                    }

                    BindValue::I64(value) => {
                        query = query.bind(value);
                    }
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
    #[test]
    fn not_eq_operator_works() {
        let query = TestUser::find().where_(TestUser::id.not_eq(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id <> $1");
    }

    #[test]
    fn gt_operator_works() {
        let query = TestUser::find().where_(TestUser::id.gt(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id > $1");
    }

    #[test]
    fn gte_operator_works() {
        let query = TestUser::find().where_(TestUser::id.gte(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id >= $1");
    }

    #[test]
    fn lt_operator_works() {
        let query = TestUser::find().where_(TestUser::id.lt(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id < $1");
    }

    #[test]
    fn lte_operator_works() {
        let query = TestUser::find().where_(TestUser::id.lte(10));
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users WHERE id <= $1");
    }
    #[test]
    fn and_condition_works() {
        let condition = TestUser::id.gt(10).and(TestUser::name.eq("Fakhir"));
        let mut next_placeholder = 1;
        let sql = compile_expression(&condition.expression, &mut next_placeholder);
        assert_eq!(sql, "id > $1 AND name = $2");
    }
    #[test]
    fn or_condition_works() {
        let condition = TestUser::id.gt(10).or(TestUser::name.eq("Fakhir"));
        let mut next_placeholder = 1;
        let sql = compile_expression(&condition.expression, &mut next_placeholder);
        assert_eq!(sql, "id > $1 OR name = $2");
    }
    #[test]
    fn order_by_asc_works() {
        let query = TestUser::find().order_by(TestUser::name.asc());
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users ORDER BY name ASC");
    }

    #[test]
    fn order_by_desc_works() {
        let query = TestUser::find().order_by(TestUser::id.desc());
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users ORDER BY id DESC");
    }
    #[test]
    fn limit_works() {
        let query = TestUser::find().take(20);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users LIMIT 20");
    }

    #[test]
    fn order_by_with_limit_works() {
        let query = TestUser::find().order_by(TestUser::name.asc()).take(20);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users ORDER BY name ASC LIMIT 20");
    }

    #[test]
    fn offset_works() {
        let query = TestUser::find().skip(40);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users OFFSET 40");
    }

    #[test]
    fn limit_with_offset_works() {
        let query = TestUser::find().take(20).skip(40);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users LIMIT 20 OFFSET 40");
    }

    #[test]
    fn order_by_limit_offset_works() {
        let query = TestUser::find()
            .order_by(TestUser::id.asc())
            .take(20)
            .skip(40);

        let sql = query.build_sql();
        assert_eq!(
            sql,
            "SELECT id, name FROM users ORDER BY id ASC LIMIT 20 OFFSET 40"
        );
    }
    #[test]
    fn distinct_works() {
        let query = TestUser::find().distinct();
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT DISTINCT id, name FROM users");
    }

    #[test]
    fn distinct_with_order_by_limit_offset_works() {
        let query = TestUser::find()
            .distinct()
            .order_by(TestUser::name.asc())
            .take(20)
            .skip(40);

        let sql = query.build_sql();

        assert_eq!(
            sql,
            "SELECT DISTINCT id, name FROM users ORDER BY name ASC LIMIT 20 OFFSET 40"
        );
    }

    #[test]
    fn group_by_works() {
        let query = TestUser::find().group_by(TestUser::name.column);
        let sql = query.build_sql();
        assert_eq!(sql, "SELECT id, name FROM users GROUP BY name");
    }
    #[test]
    fn group_by_with_order_by_limit_offset_works() {
        let query = TestUser::find()
            .group_by(TestUser::name.column)
            .order_by(TestUser::name.asc())
            .take(20)
            .skip(40);

        let sql = query.build_sql();

        assert_eq!(
            sql,
            "SELECT id, name FROM users GROUP BY name ORDER BY name ASC LIMIT 20 OFFSET 40"
        );
    }
}

use crate::query::{Condition, Query};

pub trait RelationLoader<Related> {
    fn load_relation(&mut self, related: Vec<Related>);
}

pub trait SingleRelationLoader<Related> {
    fn load_relation(&mut self, related: Option<Related>);
}

pub trait RelationKey {
    fn relation_key(&self, column: Column) -> Option<i64>;
}

#[allow(async_fn_in_trait)]
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

    async fn create<'c, D, A>(db: A, data: D) -> Result<Self::Model, sqlx::Error>
    where
        Self: Sized,
        D: crate::executor::InsertData<Self>,
        A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
        for<'r> Self::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        crate::executor::insert::<Self, D, A>(db, data).await
    }
    async fn update<'c, D, A>(db: A, id: i32, data: D) -> Result<Self::Model, sqlx::Error>
    where
        Self: Sized,
        D: crate::executor::UpdateData<Self>,
        A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
        for<'r> Self::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        crate::executor::update::<Self, D, A>(db, id, data).await
    }
    async fn delete<'c, A>(db: A, id: i32) -> Result<u64, sqlx::Error>
    where
        Self: Sized,
        A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
    {
        crate::executor::delete::<Self, A>(db, id).await
    }
    async fn create_many<'c, D, A>(db: A, data: Vec<D>) -> Result<Vec<Self::Model>, sqlx::Error>
    where
        Self: Sized,
        D: crate::executor::InsertData<Self>,
        A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
        for<'r> Self::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        crate::executor::insert_many::<Self, D, A>(db, data).await
    }
    async fn update_many<'c, D, A>(
        db: A,
        condition: Condition<Self>,
        data: D,
    ) -> Result<u64, sqlx::Error>
    where
        Self: Sized,
        D: crate::executor::UpdateData<Self>,
        A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
    {
        crate::executor::update_many::<Self, D, A>(db, condition, data).await
    }
    async fn delete_many<'c, A>(db: A, condition: Condition<Self>) -> Result<u64, sqlx::Error>
    where
        Self: Sized,
        A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
    {
        crate::executor::delete_many::<Self, A>(db, condition).await
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Column {
    table: Option<&'static str>,
    name: &'static str,
}

impl Column {
    pub const fn new(name: &'static str) -> Self {
        Self { table: None, name }
    }

    pub const fn qualified(table: &'static str, name: &'static str) -> Self {
        Self {
            table: Some(table),
            name,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn table(&self) -> Option<&'static str> {
        self.table
    }
}

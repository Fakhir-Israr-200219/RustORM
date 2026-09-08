use crate::query::Query;

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
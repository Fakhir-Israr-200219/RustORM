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
    table: Option<&'static str>,
    name: &'static str,
}

impl Column {
    pub const fn new(name: &'static str) -> Self {
        Self {
            table: None,
            name,
        }
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
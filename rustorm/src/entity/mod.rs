use crate::query::Query;

pub trait RelationLoader<Related> {
    fn load_relation(&mut self, related: Vec<Related>);
}
pub trait RelationAccess<Related> {
    fn related(&self) -> &[Related];
    fn related_mut(&mut self) -> &mut [Related];
}
pub trait SingleRelationLoader<Related> {
    fn load_relation(&mut self, related: Option<Related>);
}

pub trait RelationKey {
    fn relation_key(&self, column: Column) -> Option<i64>;
}

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

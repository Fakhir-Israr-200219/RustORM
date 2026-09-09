pub mod postgres;

pub(crate) use postgres::{collect_bind_values, compile_expression};

pub(crate) struct CompiledQuery {
    pub(crate) sql: String,
    pub(crate) binds: Vec<crate::value::BindValue>,
}

impl CompiledQuery {
    pub(crate) fn new(
        sql: String,
        binds: Vec<crate::value::BindValue>,
    ) -> Self {
        Self { sql, binds }
    }
}
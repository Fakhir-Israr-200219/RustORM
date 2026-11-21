pub mod entity;
pub mod executor;
pub mod field;
pub mod query;
pub mod sql;
pub mod types;  // ← ADD THIS
pub mod value;
pub use serde;
pub mod error;
pub use error::{Error, MigrationError, Result};
// Re-export wrapper types at crate root
pub use types::{DateTime, Decimal, JsonValue};

// Remove these old re-exports if present:
// pub use chrono;      ← DELETE
// pub use serde;       ← DELETE  
// pub use serde_json;  ← DELETE
// pub use rust_decimal;← DELETE

pub use entity::{Column, Entity};
pub use field::Field;
pub use query::Query;

#[cfg(test)]
mod tests;
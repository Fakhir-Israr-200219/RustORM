use std::fmt;

/// Top-level error type for all RustORM operations
#[derive(Debug)]
pub enum Error {
    /// Database-level error from SQLx
    Database(sqlx::Error),

    /// Entity not found by primary key
    NotFound { entity: &'static str, id: i64 },

    /// Unique constraint violation
    UniqueViolation {
        entity: &'static str,
        field: &'static str,
        value: String,
    },

    /// Foreign key constraint violation
    ForeignKeyViolation {
        entity: &'static str,
        field: &'static str,
        referenced_entity: &'static str,
    },

    /// Migration error
    Migration(MigrationError),

    /// Schema/code generation error
    Generation(String),
}

#[derive(Debug)]
pub enum MigrationError {
    /// Migration file not found
    FileNotFound(String),

    /// Checksum mismatch (schema drift detected)
    DriftDetected {
        migration: String,
        expected_checksum: String,
        actual_checksum: String,
    },

    /// Migration already applied with different content
    AlreadyAppliedDifferent(String),

    /// Failed to execute migration SQL
    ExecutionFailed {
        migration: String,
        source: sqlx::Error,
    },

    /// Failed to create tracking table
    TrackingTableFailed(sqlx::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(e) => write!(f, "Database error: {e}"),
            Error::NotFound { entity, id } => {
                write!(f, "{entity} with id {id} not found")
            }
            Error::UniqueViolation {
                entity,
                field,
                value,
            } => {
                write!(
                    f,
                    "Unique constraint violated on {entity}.{field}: '{value}' already exists"
                )
            }
            Error::ForeignKeyViolation {
                entity,
                field,
                referenced_entity,
            } => {
                write!(
                    f,
                    "Foreign key violation on {entity}.{field}: referenced {referenced_entity} does not exist"
                )
            }
            Error::Migration(e) => write!(f, "Migration error: {e}"),
            Error::Generation(msg) => write!(f, "Code generation error: {msg}"),
        }
    }
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MigrationError::FileNotFound(path) => {
                write!(f, "Migration file not found: {path}")
            }
            MigrationError::DriftDetected {
                migration,
                expected_checksum,
                actual_checksum,
            } => {
                write!(
                    f,
                    "Schema drift detected in '{migration}': expected checksum {expected_checksum}, got {actual_checksum}. \
                     The migration file was modified after being applied."
                )
            }
            MigrationError::AlreadyAppliedDifferent(name) => {
                write!(
                    f,
                    "Migration '{name}' already applied with different content"
                )
            }
            MigrationError::ExecutionFailed { migration, source } => {
                write!(f, "Failed to execute migration '{migration}': {source}")
            }
            MigrationError::TrackingTableFailed(e) => {
                write!(f, "Failed to create migration tracking table: {e}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Database(e) => Some(e),
            Error::Migration(MigrationError::ExecutionFailed { source, .. }) => Some(source),
            Error::Migration(MigrationError::TrackingTableFailed(e)) => Some(e),
            _ => None,
        }
    }
}

impl std::error::Error for MigrationError {}

// Automatic conversion from sqlx::Error
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        // Try to classify the database error
        match &err {
            sqlx::Error::RowNotFound => Error::NotFound {
                entity: "unknown",
                id: -1,
            },
            sqlx::Error::Database(db_err) => {
                let code = db_err.code().map(|c| c.to_string()).unwrap_or_default();
                let message = db_err.message();

                // PostgreSQL error codes
                // 23505 = unique_violation
                // 23503 = foreign_key_violation
                // ✅ NAYA
                if code == "23505" {
                    let (entity, field) = parse_unique_violation(message);
                    Error::UniqueViolation {
                        entity: Box::leak(entity.into_boxed_str()),
                        field: Box::leak(field.into_boxed_str()),
                        value: message.to_string(),
                    }
                } else if code == "23503" {
                    let (entity, field) = parse_fk_violation(message);
                    Error::ForeignKeyViolation {
                        entity: Box::leak(entity.into_boxed_str()),
                        field: Box::leak(field.into_boxed_str()),
                        referenced_entity: "unknown",
                    }
                } else {
                    Error::Database(err)
                }
            }
            _ => Error::Database(err),
        }
    }
}

impl From<MigrationError> for Error {
    fn from(err: MigrationError) -> Self {
        Error::Migration(err)
    }
}

/// Convenience Result type alias
pub type Result<T> = std::result::Result<T, Error>;

// ==================== PG ERROR PARSERS ====================

fn parse_unique_violation(msg: &str) -> (String, String) {
    // Pattern: ...constraint "users_email_key"
    if let Some(constraint) = msg.split('"').nth(1)
        && let Some(field) = constraint.strip_suffix("_key")
        && let Some(table_end) = field.rfind('_')
    {
        return (
            field[..table_end].to_string(),
            field[table_end + 1..].to_string(),
        );
    }
    ("unknown".to_string(), "unknown".to_string())
}

fn parse_fk_violation(msg: &str) -> (String, String) {
    let table = msg
        .split("table \"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap_or("unknown")
        .to_string();

    let field = msg
        .split("constraint \"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .and_then(|c| c.strip_suffix("_fkey"))
        .and_then(|c| c.find('_').map(|i| &c[i + 1..]))
        .unwrap_or("unknown")
        .to_string();

    (table, field)
}

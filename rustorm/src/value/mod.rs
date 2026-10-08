#[derive(Clone)]
pub enum BindValue {
    String(String),
    I64(i64),
    Boolean(bool),
    F64(f64),
    DateTime(chrono::NaiveDateTime),
    Decimal(rust_decimal::Decimal),
    Json(serde_json::Value),
    Null,
}
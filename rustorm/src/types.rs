//! Wrapper types with built-in serde + sqlx support.

use std::fmt;
// use std::str::FromStr;

// ==================== DateTime ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime(pub chrono::NaiveDateTime);

impl DateTime {
    pub fn now() -> Self {
        Self(chrono::Utc::now().naive_utc())
    }

    pub fn into_inner(self) -> chrono::NaiveDateTime {
        self.0
    }
}

impl From<chrono::NaiveDateTime> for DateTime {
    fn from(dt: chrono::NaiveDateTime) -> Self {
        Self(dt)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S"))
    }
}

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for DateTime {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <chrono::NaiveDateTime as sqlx::Decode<'r, sqlx::Postgres>>::decode(value).map(Self)
    }
}

impl sqlx::Type<sqlx::Postgres> for DateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <chrono::NaiveDateTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for DateTime {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <chrono::NaiveDateTime as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}

// ==================== Decimal ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decimal(pub rust_decimal::Decimal);

impl Decimal {
    pub fn new(mantissa: i64, scale: u32) -> Self {
        Self(rust_decimal::Decimal::new(mantissa, scale))
    }

    pub fn into_inner(self) -> rust_decimal::Decimal {
        self.0
    }
}

impl From<rust_decimal::Decimal> for Decimal {
    fn from(d: rust_decimal::Decimal) -> Self {
        Self(d)
    }
}

impl serde::Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        rust_decimal::Decimal::from_str_exact(&s)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Decimal {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <rust_decimal::Decimal as sqlx::Decode<'r, sqlx::Postgres>>::decode(value).map(Self)
    }
}

impl sqlx::Type<sqlx::Postgres> for Decimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <rust_decimal::Decimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Decimal {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <rust_decimal::Decimal as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}

// ==================== JsonValue ====================

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonValue(pub serde_json::Value);

impl JsonValue {
    pub fn into_inner(self) -> serde_json::Value {
        self.0
    }
}

impl From<serde_json::Value> for JsonValue {
    fn from(v: serde_json::Value) -> Self {
        Self(v)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for JsonValue {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <serde_json::Value as sqlx::Decode<'r, sqlx::Postgres>>::decode(value).map(Self)
    }
}

impl sqlx::Type<sqlx::Postgres> for JsonValue {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for JsonValue {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <serde_json::Value as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}
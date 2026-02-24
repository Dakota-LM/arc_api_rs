use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use url::Url;
use uuid::Uuid;

/// Helper to deserialize either a string or array of strings into a Vector of String values
pub fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    match Option::<StringOrVec>::deserialize(deserializer)? {
        None => Ok(None),
        Some(StringOrVec::String(s)) => Ok(Some(vec![s])),
        Some(StringOrVec::Vec(v)) => Ok(Some(v)),
    }
}

/// A validated UUID string that ensures the format is correct
#[derive(Debug, Clone, Serialize)]
pub struct UuidString(pub String);

impl<'de> Deserialize<'de> for UuidString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // Validate it's a proper UUID format
        Uuid::from_str(&s)
            .map_err(|e| serde::de::Error::custom(format!("Invalid UUID format: {}", e)))?;
        Ok(UuidString(s))
    }
}

/// A validated RFC3339 timestamp string that wraps OffsetDateTime
#[derive(Debug, Clone)]
pub struct DateTimeString(pub OffsetDateTime);

impl<'de> Deserialize<'de> for DateTimeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // Parse and validate RFC3339 format
        let dt = OffsetDateTime::parse(&s, &Rfc3339)
            .map_err(|e| serde::de::Error::custom(format!("Invalid RFC3339 timestamp: {}", e)))?;
        Ok(DateTimeString(dt))
    }
}

impl Serialize for DateTimeString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize back to RFC3339 string
        let s = self
            .0
            .format(&Rfc3339)
            .map_err(|e| serde::ser::Error::custom(format!("Failed to format timestamp: {}", e)))?;
        serializer.serialize_str(&s)
    }
}

/// A validated URI/URL string that wraps url::Url
#[derive(Debug, Clone)]
pub struct UriString(pub Url);

impl<'de> Deserialize<'de> for UriString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // Parse and validate URI format
        let url = Url::parse(&s)
            .map_err(|e| serde::de::Error::custom(format!("Invalid URI format: {}", e)))?;
        Ok(UriString(url))
    }
}

impl Serialize for UriString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

/// Helper to deserialize optional URIs that might be null or empty strings
pub fn deserialize_optional_uri<'de, D>(deserializer: D) -> Result<Option<UriString>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => {
            let url = Url::parse(&s)
                .map_err(|e| serde::de::Error::custom(format!("Invalid URI format: {}", e)))?;
            Ok(Some(UriString(url)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: u32,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_prev_page: bool,
}

/// A 2D position with signed float coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Standard MetaForge list wrapper (as seen on /items).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,

    /// Some endpoints may include extra top-level fields (like maxValue).
    /// We keep this optional so other endpoints can reuse the same wrapper.
    #[serde(default)]
    pub max_value: Option<i32>,
}

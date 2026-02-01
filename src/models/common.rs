use serde::{Deserialize, Serialize, Deserializer};
use uuid::Uuid;
use std::str::FromStr;

/// Helper to deserialize either a string or array of strings into Vec<String>
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

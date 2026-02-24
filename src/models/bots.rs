use serde::{Deserialize, Serialize};
// use serde_json::Value;
use super::common::{deserialize_string_or_vec, deserialize_optional_uri, DateTimeString, UriString};


/// A bot returned by the MetaForge ARC Raiders API.
///
/// Source sample provided by you:
/// - /arcs?page=1&limit=2 returns a raw JSON array of these bots.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Bot {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub description: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_optional_uri")]
    pub icon: Option<UriString>,
    #[serde(deserialize_with = "deserialize_optional_uri")]
    pub image: Option<UriString>,
    pub created_at: DateTimeString,
    pub updated_at: Option<DateTimeString>,
}
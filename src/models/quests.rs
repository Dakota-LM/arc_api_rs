use serde::{Deserialize, Serialize};
// use serde_json::Value;
use super::common::{deserialize_string_or_vec, deserialize_optional_uri, DateTimeString, Position, UriString};


/// A quest returned by the MetaForge ARC Raiders API.
///
/// Source sample provided by you:
/// - /quests?page=1&limit=2 returns a raw JSON array of these quests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub objectives: Option<Vec<String>>,
    pub xp: i32,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub granted_items: Option<Vec<String>>,
    pub created_at: DateTimeString,
    pub updated_at: Option<DateTimeString>,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub locations: Option<Vec<String>>,
    pub marker_category: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_uri")]
    pub image: Option<UriString>,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub guide_links: Option<Vec<String>>,
    pub trader_name: Option<String>,
    pub sort_order: Option<i32>,
    pub position: Option<Position>,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub required_items: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub rewards: Option<Vec<String>>,
}
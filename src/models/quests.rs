use serde::{Deserialize, Deserializer, Serialize};
use super::common::{deserialize_string_or_vec, deserialize_optional_uri, DateTimeString, Position, UriString};

fn deserialize_quantity<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU32 {
        String(String),
        Number(u32),
    }

    match StringOrU32::deserialize(deserializer)? {
        StringOrU32::String(s) => s.parse::<u32>().map_err(serde::de::Error::custom),
        StringOrU32::Number(n) => Ok(n),
    }
}

/// The item snapshot embedded inside a quest reward.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRewardItem {
    pub id: String,
    pub name: String,
    pub rarity: Option<String>,
    pub item_type: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_uri")]
    pub icon: Option<UriString>,
}

/// A reward granted upon quest completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestReward {
    pub id: String,
    pub item: Option<QuestRewardItem>,
    pub item_id: String,
    #[serde(deserialize_with = "deserialize_quantity")]
    pub quantity: u32,
}

/// A guide link associated with a quest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestGuideLink {
    pub url: String,
    pub label: String,
}

/// A quest returned by the MetaForge ARC Raiders API.
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
    pub guide_links: Option<Vec<QuestGuideLink>>,
    pub trader_name: Option<String>,
    pub sort_order: Option<i32>,
    pub position: Option<Position>,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub required_items: Option<Vec<String>>,
    pub rewards: Option<Vec<QuestReward>>,
}

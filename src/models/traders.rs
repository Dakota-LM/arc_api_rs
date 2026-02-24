use serde::{Deserialize, Serialize};
// use serde_json::Value;
use super::common::{deserialize_string_or_vec, deserialize_optional_uri, DateTimeString, UriString};


/// A trader returned by the MetaForge ARC Raiders API.
///
/// Source sample provided by you:
/// - /traders returns a raw JSON array of these traders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Trader {
    pub apollo: Option<Vec<TraderItem>>,
    pub celeste: Option<Vec<TraderItem>>,
    pub lance: Option<Vec<TraderItem>>,
    pub shani: Option<Vec<TraderItem>>,
    pub tian_wen: Option<Vec<TraderItem>>,
}

pub struct TraderItem {
    pub id: String,
    pub icon: UriString,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub name: Option<Vec<String>>,
    pub value: i32,
    pub rarity: String,
    pub item_type: Option<Vec<ItemType>>,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub description: Option<Vec<String>>,
    pub trader_price: i32,
}

pub enum ItemType {
    Modification,
    Weapon,
    Ammunition,
    Gadget,
    Key,
    Quick Use,
    Augment,
    Shield,
    Topside Material,
    Basic Material,
    Recyclable
}
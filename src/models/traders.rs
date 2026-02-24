use serde::{Deserialize, Serialize};
use super::common::UriString;

/// A single item sold by a trader.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraderItem {
    pub id: String,
    pub icon: UriString,
    pub name: String,
    pub value: i32,
    pub rarity: String,
    pub item_type: String,
    pub description: String,
    pub trader_price: i32,
}

/// The full traders response from the MetaForge ARC Raiders API.
///
/// Each field corresponds to one trader and contains their inventory.
/// Trader names are PascalCase in the API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradersResponse {
    pub success: bool,
    pub data: TraderInventories,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TraderInventories {
    pub apollo: Option<Vec<TraderItem>>,
    pub celeste: Option<Vec<TraderItem>>,
    pub lance: Option<Vec<TraderItem>>,
    pub shani: Option<Vec<TraderItem>>,
    pub tian_wen: Option<Vec<TraderItem>>,
}

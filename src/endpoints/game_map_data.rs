use crate::error::MetaForgeError;
use crate::models::*;
use crate::models::map::MapDataResponse;
use crate::MetaForgeClient;
use serde::Serialize;

pub const TABLE_ID : &str = "arc_map_data";

#[derive(Debug, Clone, Serialize)]
// #[serde(rename_all = "snake_case")]
pub struct MapDataQuery {
    #[serde(rename = "tableID")]
    pub table_id: String,
    #[serde(rename = "mapID")]
    pub map_id: Map,
}

impl MetaForgeClient {
    /// Fetch the current map data.
    pub async fn map_data(&self, q: &MapDataQuery) -> Result<MapDataResponse, MetaForgeError> {
        println!("Fetching map data for table_id='{}', map_id='{:?}'", q.table_id, q.map_id);
        self.get_json_with_query("game-map-data", q).await
    }
}
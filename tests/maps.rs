use arc_api_rs::models::Map;
use arc_api_rs::{MapDataQuery, MetaForgeClient};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn map_json(id: &str, name: &str) -> serde_json::Value {
    let template = r#"
{
  "id": "__ID__",
  "lat": 12.34,
  "lng": 56.78,
  "zlayers": 2,
  "mapID": "buried-city",
  "category": "locations",
  "subcategory": "raider_camp",
  "instanceName": ["Instance1", "Instance2"],
  "added_by": ["UserA"],
  "behindLockedDoor": false,
  "last_edited_by": ["UserB"],
  "updated_at": "2024-01-01T00:00:00Z",
  "eventConditionMask": 0,
  "lootAreas": ["Old World", "New World"]
}
"#;

    let s = template.replace("__ID__", id).replace("__NAME__", name);
    serde_json::from_str(&s).expect("valid json")
}

#[tokio::test]
async fn maps_sends_query_and_deserializes() {
    let server = MockServer::start().await;

    // Mock expects GET /game-map-data?tableID=arc_map_data&mapID=buried-city
    Mock::given(method("GET"))
        .and(path("/game-map-data"))
        .and(query_param("tableID", "arc_map_data"))
        .and(query_param("mapID", "buried-city"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "allData": [
                map_json("550e8400-e29b-41d4-a716-446655440000", "Buried City"),
                map_json("550e8400-e29b-41d4-a716-446655440001", "Raider Camp")
            ]
        })))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = MetaForgeClient::with_client_and_base(http, server.uri());

    let q = MapDataQuery {
        table_id: "arc_map_data".to_string(),
        map_id: Map::BuriedCity,
    };

    let resp = client.map_data(&q).await.expect("game_map_data failed");

    assert_eq!(resp.all_data.len(), 2);
    assert_eq!(resp.all_data[0].id.0, "550e8400-e29b-41d4-a716-446655440000");
}
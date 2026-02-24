use arc_api_rs::MetaForgeClient;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn trader_item_json(id: &str, name: &str) -> serde_json::Value {
    json!({
        "id": id,
        "icon": format!("https://cdn.metaforge.app/arc-raiders/icons/{}.webp", id),
        "name": name,
        "value": 640,
        "rarity": "Uncommon",
        "item_type": "Quick Use",
        "description": "A test item.",
        "trader_price": 1920
    })
}

#[tokio::test]
async fn traders_deserializes() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/arc-raiders/traders"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "Apollo":  [ trader_item_json("barricade-kit", "Barricade Kit") ],
                "Celeste": [ trader_item_json("med-kit", "Med Kit") ],
                "Lance":   [],
                "Shani":   [],
                "TianWen": [ trader_item_json("stable-stock-i", "Stable Stock I") ]
            }
        })))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = MetaForgeClient::with_client_and_base(http, server.uri());

    let resp = client.traders().await.expect("traders failed");

    assert!(resp.success);

    let apollo = resp.data.apollo.as_ref().unwrap();
    assert_eq!(apollo.len(), 1);
    assert_eq!(apollo[0].id, "barricade-kit");
    assert_eq!(apollo[0].trader_price, 1920);

    let tian_wen = resp.data.tian_wen.as_ref().unwrap();
    assert_eq!(tian_wen[0].id, "stable-stock-i");

    assert_eq!(resp.data.lance.as_ref().unwrap().len(), 0);
}

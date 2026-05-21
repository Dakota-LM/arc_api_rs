use arc_api_rs::{ItemsQuery, MetaForgeClient};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn item_json(id: &str, name: &str) -> serde_json::Value {
    let template = r#"
{
  "id": "__ID__",
  "name": "__NAME__",
  "description": "test item",
  "item_type": "Quick Use",
  "loadout_slots": [],
  "icon": "https://cdn.example/icon.webp",
  "rarity": "Common",
  "value": 123,
  "workbench": null,
  "stat_block": {
    "range": 0.0,
    "value": 0.0,
    "damage": 0.0,
    "health": 0.0,
    "radius": 0.0,
    "shield": 0.0,
    "weight": 0.0,
    "agility": 0.0,
    "arcStun": 0.0,
    "healing": 0.0,
    "stamina": 0.0,
    "stealth": 0.0,
    "useTime": 0.0,
    "duration": 0.0,
    "fireRate": 0.0,
    "stability": 0.0,
    "stackSize": 1.0,
    "damageMult": 0.0,
    "raiderStun": 0.0,
    "weightLimit": 0.0,
    "augmentSlots": 0.0,
    "healingSlots": 0.0,
    "magazineSize": 0.0,
    "reducedNoise": 0.0,
    "shieldCharge": 0.0,
    "backpackSlots": 0.0,
    "quickUseSlots": 0.0,
    "damagePerSecond": 0.0,
    "movementPenalty": 0.0,
    "safePocketSlots": 0.0,
    "damageMitigation": 0.0,
    "healingPerSecond": 0.0,
    "reducedEquipTime": 0.0,
    "staminaPerSecond": 0.0,
    "increasedADSSpeed": 0.0,
    "increasedFireRate": 0.0,
    "reducedReloadTime": 0.0,
    "illuminationRadius": 0.0,
    "increasedEquipTime": 0.0,
    "reducedUnequipTime": 0.0,
    "shieldCompatibility": "",
    "increasedUnequipTime": 0.0,
    "reducedVerticalRecoil": 0.0,
    "increasedBulletVelocity": 0.0,
    "increasedVerticalRecoil": 0.0,
    "reducedMaxShotDispersion": 0.0,
    "reducedPerShotDispersion": 0.0,
    "reducedDurabilityBurnRate": 0.0,
    "reducedRecoilRecoveryTime": 0.0,
    "increasedRecoilRecoveryTime": 0.0,
    "reducedDispersionRecoveryTime": 0.0
  },
  "flavor_text": null,
  "subcategory": null,
  "created_at": "2026-01-01T00:00:00+00:00",
  "updated_at": "2026-01-01T00:00:00+00:00",
  "shield_type": null,
  "loot_area": null,
  "sources": null,
  "ammo_type": null,
  "locations": [],
  "guide_links": [],
  "article": null,
  "guide_url": null
}
"#;

    let s = template.replace("__ID__", id).replace("__NAME__", name);
    serde_json::from_str(&s).expect("valid json")
}


#[tokio::test]
async fn items_paged_sends_query_and_deserializes() {
    let server = MockServer::start().await;

    // Mock expects GET /arc-raiders/items?page=1&limit=2
    Mock::given(method("GET"))
        .and(path("/arc-raiders/items"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [
                item_json("acoustic-guitar", "Acoustic Guitar"),
                item_json("adrenaline-shot", "Adrenaline Shot")
            ],
            "maxValue": 27500,
            "pagination": {
                "page": 1,
                "limit": 2,
                "total": 527,
                "totalPages": 264,
                "hasNextPage": true,
                "hasPrevPage": false
            }
        })))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = MetaForgeClient::with_client_and_base(http, server.uri());

    let q = ItemsQuery {
        page: Some(1),
        limit: Some(2),
        ..Default::default()
    };

    let resp = client.items_paged(&q).await.expect("items_paged failed");

    assert_eq!(resp.pagination.page, 1);
    assert_eq!(resp.pagination.limit, 2);
    assert_eq!(resp.pagination.total_pages, 264);
    assert_eq!(resp.max_value, Some(27500));

    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].id, "acoustic-guitar");
}

#[tokio::test]
async fn item_by_id_returns_some() {
    let server = MockServer::start().await;

    // Your helper calls /arc-raiders/items?id=...&page=1&limit=1 (based on what we wrote)
    Mock::given(method("GET"))
        .and(path("/arc-raiders/items"))
        .and(query_param("id", "acoustic-guitar"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [ item_json("acoustic-guitar", "Acoustic Guitar") ],
            "maxValue": 27500,
            "pagination": {
                "page": 1,
                "limit": 1,
                "total": 1,
                "totalPages": 1,
                "hasNextPage": false,
                "hasPrevPage": false
            }
        })))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = MetaForgeClient::with_client_and_base(http, server.uri());

    let item = client.item_by_id("acoustic-guitar").await.unwrap();
    assert!(item.is_some());
    assert_eq!(item.unwrap().name, "Acoustic Guitar");
}

#[tokio::test]
async fn item_by_id_returns_none_when_empty() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/arc-raiders/items"))
        .and(query_param("id", "does-not-exist"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [],
            "pagination": {
                "page": 1,
                "limit": 1,
                "total": 0,
                "totalPages": 0,
                "hasNextPage": false,
                "hasPrevPage": false
            }
        })))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = MetaForgeClient::with_client_and_base(http, server.uri());

    let item = client.item_by_id("does-not-exist").await.unwrap();
    assert!(item.is_none());
}

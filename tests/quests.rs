use arc_api_rs::{QuestQuery, MetaForgeClient};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn quest_json(id: &str, name: &str) -> serde_json::Value {
    json!({
        "id": id,
        "name": name,
        "objectives": ["Reach the extraction point", "Collect the data drive"],
        "xp": 500,
        "granted_items": null,
        "created_at": "2026-01-01T00:00:00+00:00",
        "updated_at": "2026-01-01T00:00:00+00:00",
        "locations": ["Buried City"],
        "marker_category": null,
        "image": null,
        "guide_links": null,
        "trader_name": "Apollo",
        "sort_order": 1,
        "position": null,
        "required_items": null,
        "rewards": [
            {
                "id": "152eefaf-732f-4723-9ef4-46e1a28a8ec2",
                "item": {"id": "green-light-stick", "name": "Green Light Stick", "rarity": "Common", "item_type": "Quick Use", "icon": "https://cdn.metaforge.app/arc-raiders/icons/green-light-stick.webp"},
                "item_id": "green-light-stick",
                "quantity": "5"
            },
            {
                "id": "7fe4d5f7-1f1f-406e-a87e-8046e5be0c31",
                "item": {"id": "looting-mk-2", "name": "Looting Mk.2", "rarity": "Uncommon", "item_type": "Augment", "icon": "https://cdn.metaforge.app/arc-raiders/icons/looting-mk-2.webp"},
                "item_id": "looting-mk-2",
                "quantity": "1"
            }
        ]
    })
}

#[tokio::test]
async fn quests_paged_sends_query_and_deserializes() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/arc-raiders/quests"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [
                quest_json("what-we-left-behind", "What We Left Behind"),
                quest_json("medical-merchandise", "Medical Merchandise")
            ],
            "pagination": {
                "page": 1,
                "limit": 2,
                "total": 12,
                "totalPages": 6,
                "hasNextPage": true,
                "hasPrevPage": false
            }
        })))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = MetaForgeClient::with_client_and_base(http, server.uri());

    let q = QuestQuery {
        page: Some(1),
        limit: Some(2),
        ..Default::default()
    };

    let resp = client.quests_paged(&q).await.expect("quests_paged failed");

    assert_eq!(resp.pagination.page, 1);
    assert_eq!(resp.pagination.limit, 2);
    assert_eq!(resp.pagination.total_pages, 6);
    assert_eq!(resp.max_value, None);

    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].id, "what-we-left-behind");
    assert_eq!(resp.data[0].xp, 500);
    assert_eq!(resp.data[0].trader_name.as_deref(), Some("Apollo"));
    assert!(resp.data[0].granted_items.is_none());
    let rewards = resp.data[0].rewards.as_ref().unwrap();
    assert_eq!(rewards.len(), 2);
    assert_eq!(rewards[0].item_id, "green-light-stick");
    assert_eq!(rewards[0].quantity, 5);
}

#[tokio::test]
async fn quest_by_id_returns_some() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/arc-raiders/quests"))
        .and(query_param("id", "what-we-left-behind"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [ quest_json("what-we-left-behind", "What We Left Behind") ],
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

    let quest = client.quest_by_id("what-we-left-behind").await.unwrap();
    assert!(quest.is_some());
    assert_eq!(quest.unwrap().name, "What We Left Behind");
}

#[tokio::test]
async fn quest_by_id_returns_none_when_empty() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/arc-raiders/quests"))
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

    let quest = client.quest_by_id("does-not-exist").await.unwrap();
    assert!(quest.is_none());
}

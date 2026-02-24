use arc_api_rs::{ArcsQuery, MetaForgeClient};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

fn arc_json(id: &str, name: &str, has_image: bool) -> serde_json::Value {
    let icon_url = format!("https://cdn.example/icons/{}.webp", id);
    let image_value = if has_image {
        json!(format!("https://cdn.example/images/{}.webp", id))
    } else {
        json!("")
    };
    
    json!({
        "id": id,
        "name": name,
        "description": "A test ARC unit description.",
        "icon": icon_url,
        "image": image_value,
        "created_at": "2026-01-01T00:00:00+00:00",
        "updated_at": "2026-01-01T00:00:00+00:00"
    })
}

#[tokio::test]
async fn arcs_paged_sends_query_and_deserializes() {
    let server = MockServer::start().await;

    // Mock expects GET /arc-raiders/arcs?page=1&limit=2
    Mock::given(method("GET"))
        .and(path("/arc-raiders/arcs"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [
                arc_json("bastion", "Bastion", true),
                arc_json("bombardier", "Bombardier", false)
            ],
            "pagination": {
                "page": 1,
                "limit": 2,
                "total": 17,
                "totalPages": 9,
                "hasNextPage": true,
                "hasPrevPage": false
            }
        })))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = MetaForgeClient::with_client_and_base(http, server.uri());

    let q = ArcsQuery {
        page: Some(1),
        limit: Some(2),
        ..Default::default()
    };

    let resp = client.arcs_paged(&q).await.expect("arcs_paged failed");

    assert_eq!(resp.pagination.page, 1);
    assert_eq!(resp.pagination.limit, 2);
    assert_eq!(resp.pagination.total_pages, 9);
    assert_eq!(resp.max_value, None);

    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].id, "bastion");
    assert!(resp.data[0].icon.is_some());
    assert!(resp.data[0].image.is_some());
    
    assert_eq!(resp.data[1].id, "bombardier");
    assert!(resp.data[1].icon.is_some());
    // bombardier has empty string for image, should be None
    assert!(resp.data[1].image.is_none());
}

#[tokio::test]
async fn arc_by_id_returns_some() {
    let server = MockServer::start().await;

    // Your helper calls /arc-raiders/arcs?id=...&page=1&limit=1
    Mock::given(method("GET"))
        .and(path("/arc-raiders/arcs"))
        .and(query_param("id", "bastion"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [ arc_json("bastion", "Bastion", true) ],
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

    let arc = client.arc_by_id("bastion").await.unwrap();
    assert!(arc.is_some());
    assert_eq!(arc.unwrap().name, "Bastion");
}

#[tokio::test]
async fn arc_by_id_returns_none_when_empty() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/arc-raiders/arcs"))
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

    let arc = client.arc_by_id("does-not-exist").await.unwrap();
    assert!(arc.is_none());
}

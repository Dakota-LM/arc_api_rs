#![recursion_limit = "256"]

use arc_api_rs::MetaForgeClient;
use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn events_schedule_deserializes() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/events-schedule"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [
                {
                    "name": "Night Raid",
                    "map": "Dam",
                    "icon": "https://cdn.example/night.webp",
                    "startTime": 1769824800000i64,
                    "endTime": 1769828400000i64
                }
            ],
            "cachedAt": 1769830603750i64
        })))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = MetaForgeClient::with_client_and_base(http, server.uri());

    let sched = client.events_schedule().await.expect("events_schedule failed");

    assert_eq!(sched.cached_at, 1769830603750);
    assert_eq!(sched.data.len(), 1);
    assert_eq!(sched.data[0].name, "Night Raid");
    assert_eq!(sched.data[0].map, "Dam");
}

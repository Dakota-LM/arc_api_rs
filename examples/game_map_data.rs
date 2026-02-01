use arc_api_rs::MetaForgeClient;
use arc_api_rs::models::Map;
use arc_api_rs::endpoints::{MapDataQuery, TABLE_ID};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MetaForgeClient::new();

    let response = client.map_data(&MapDataQuery {
        table_id: TABLE_ID.to_string(),
        map_id: Map::BuriedCity,
    }).await?;

    let json = serde_json::to_string_pretty(&response)?;
    println!("map data: {}", json);

    println!("\nTotal items: {}", response.all_data.len());

    // // Print first 10 entries
    // for e in response.all_data.iter().take(10) {
    //     println!(
    //         "{} @ {} | {} -> {}",
    //         e.name, e.map, e.start_time, e.end_time
    //     );
    // }

    Ok(())
}
use arc_api_rs::models::{Item, PagedResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http = reqwest::Client::new();

    for page in 1..=7 {
        let raw = http
            .get("https://metaforge.app/api/arc-raiders/items")
            .query(&[
                ("page", &page.to_string()),
                ("limit", &"100".to_string()),
                ("includeComponents", &"true".to_string()),
            ])
            .send()
            .await?
            .text()
            .await?;

        match serde_json::from_str::<PagedResponse<Item>>(&raw) {
            Ok(resp) => {
                println!("Page {}: OK ({} items)", page, resp.data.len());
                if !resp.pagination.has_next_page {
                    println!("No more pages.");
                    break;
                }
            }
            Err(e) => {
                println!("Page {}: FAILED - {}", page, e);
                let v: serde_json::Value = serde_json::from_str(&raw)?;
                let items = v["data"].as_array().unwrap();
                for (i, item_val) in items.iter().enumerate() {
                    let item_json = serde_json::to_string(item_val)?;
                    if let Err(ie) = serde_json::from_str::<Item>(&item_json) {
                        let name = item_val["name"].as_str().unwrap_or("?");
                        println!("  Item {}: '{}' - {}", i, name, ie);
                    }
                }
                // Don't break, check all pages
            }
        }
    }

    Ok(())
}

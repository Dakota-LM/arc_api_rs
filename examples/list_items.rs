use arc_api_rs::{MetaForgeClient, ItemsQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MetaForgeClient::new();

    let q = ItemsQuery {
        page: Some(1),
        limit: Some(2),
        ..Default::default()
    };

    let resp = client.items_paged(&q).await?;

    println!(
        "page {} / {} (total {}, has_next={}) max_value={:?}",
        resp.pagination.page,
        resp.pagination.total_pages,
        resp.pagination.total,
        resp.pagination.has_next_page,
        resp.max_value
    );

    for item in resp.data {
        println!(
            "{} | {} | {} | value={}",
            item.id, item.name, item.rarity, item.value
        );
    }

    Ok(())
}

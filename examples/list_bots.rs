use arc_api_rs::{MetaForgeClient, BotsQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MetaForgeClient::new();

    let q = BotsQuery {
        page: Some(1),
        limit: Some(2),
        ..Default::default()
    };

    let resp = client.bots_paged(&q).await?;

    println!(
        "page {} / {} (total {}, has_next={}) max_value={:?}",
        resp.pagination.page,
        resp.pagination.total_pages,
        resp.pagination.total,
        resp.pagination.has_next_page,
        resp.max_value
    );

    for bot in resp.data {
        println!(
            "{} | {} | Description={:?}",
            bot.id, bot.name, bot.description
        );
    }

    Ok(())
}

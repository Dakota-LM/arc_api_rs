use arc_api_rs::{MetaForgeClient, QuestQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MetaForgeClient::new();

    let q = QuestQuery {
        page: Some(1),
        limit: Some(2),
        ..Default::default()
    };

    let resp = client.quests_paged(&q).await?;

    println!(
        "page {} / {} (total {}, has_next={})",
        resp.pagination.page,
        resp.pagination.total_pages,
        resp.pagination.total,
        resp.pagination.has_next_page,
    );

    for quest in resp.data {
        println!(
            "{} | {} | xp={} | trader={:?}",
            quest.id, quest.name, quest.xp, quest.trader_name
        );
    }

    Ok(())
}

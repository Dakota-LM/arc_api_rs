use arc_api_rs::MetaForgeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MetaForgeClient::new();

    let resp = client.traders().await?;

    let traders = [
        ("Apollo",   resp.data.apollo.as_deref()),
        ("Celeste",  resp.data.celeste.as_deref()),
        ("Lance",    resp.data.lance.as_deref()),
        ("Shani",    resp.data.shani.as_deref()),
        ("Tian Wen", resp.data.tian_wen.as_deref()),
    ];

    for (trader_name, items) in traders {
        let items = match items {
            Some(i) => i,
            None => continue,
        };
        println!("{} ({} items):", trader_name, items.len());
        for item in items {
            println!("  {} | {} | {} credits", item.id, item.name, item.trader_price);
        }
    }

    Ok(())
}

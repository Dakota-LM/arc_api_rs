use arc_api_rs::MetaForgeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MetaForgeClient::new();

    let sched = client.events_schedule().await?;

    println!("cached_at(ms): {}", sched.cached_at);
    println!("events: {}", sched.data.len());

    // Print first 10 entries
    for e in sched.data.iter().take(10) {
        println!(
            "{} @ {} | {} -> {}",
            e.name, e.map, e.start_time, e.end_time
        );
    }

    Ok(())
}

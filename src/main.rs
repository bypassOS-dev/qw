use tracing::Instrument;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let task = handle_client(1);
    let task1 = handle_client(2);

    tokio::join!(task, task1);
    println!("All task is created!");
}
async fn handle_client(id: i32) {
    let span = tracing::info_span!("client", id = id);
    async {
        tracing::info!("Client connected!");
        tokio::time::sleep(tokio::time::Duration::from_millis(123)).await;
        tracing::info!("client disconnected!");
    }
    .instrument(span)
    .await
}

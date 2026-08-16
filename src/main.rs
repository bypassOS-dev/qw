use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    let span = tracing::info_span!("client", id = 1);
    let _guard = span.enter();
    info!("Hi");
}

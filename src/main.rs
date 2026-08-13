use tokio_retry::{Retry, strategy::{ExponentialBackoff, jitter}};

async fn fetch_user_from_api(user_id: i32, attemps: &mut i32) {
    *attemps += 1;

}
async fn download_some(user_id: i32) {
    let mut attemps = 0;

    let strategy = ExponentialBackoff::from_millis(100)
        .map(jitter)
        .take(3);    
    let jons_data = Retry::spawn(strategy, || async {
        fetch_user_from_api(user_id,&mut attemps).await
    }).await;
}

#[tokio::main]
async fn main() {
    let user_id = 123;
    let path = download_some(user_id).await;

}

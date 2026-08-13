
async fn fetch_user_from_api(user_id: i32, mut attemps: i32) {
    attemps += 1;

}
async fn download_some(user_id: i32) -> String{
    todo!()
}

#[tokio::main]
async fn main() {
    let user_id = 123;
    let path = download_some(user_id).await;
}
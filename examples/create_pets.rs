use petstore::PetstoreClient;
#[tokio::main]
async fn main() {
    let client = PetstoreClient::from_env();
    let response = client.create_pets().send().await.unwrap();
    println!("{:#?}", response);
}

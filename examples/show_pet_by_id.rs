use petstore::PetstoreClient;
#[tokio::main]
async fn main() {
    let client = PetstoreClient::from_env();
    let pet_id = 1;
    let response = client.show_pet_by_id(pet_id).send().await.unwrap();
    println!("{:#?}", response);
}

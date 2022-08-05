use petstore::PetStoreClient;

fn main() {
    let client = PetStoreClient::new("http://petstore.swagger.io/v2");
    client.list_pets().send();
}
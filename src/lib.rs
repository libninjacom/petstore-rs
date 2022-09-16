//! [`Petstore Client`](struct.Petstore Client.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct PetstoreClient {
    pub(crate) client: httpclient::Client,
}
impl PetstoreClient {
    pub fn from_env() -> Self {
        let url = std::env::var("PETSTORE_BASE_URL")
            .expect("Missing environment variable PETSTORE_BASE_URL");
        Self::new(&url)
    }
}
impl PetstoreClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        PetstoreClient { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///List all pets
    pub fn list_pets(&self) -> request::ListPetsRequest {
        request::ListPetsRequest {
            client: &self,
            limit: None,
        }
    }
    ///Create a pet
    pub fn create_pets(&self) -> request::CreatePetsRequest {
        request::CreatePetsRequest {
            client: &self,
        }
    }
    ///Info for a specific pet
    pub fn show_pet_by_id(&self, pet_id: i64) -> request::ShowPetByIdRequest {
        request::ShowPetByIdRequest {
            client: &self,
            pet_id,
        }
    }
}

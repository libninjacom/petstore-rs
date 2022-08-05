//! [`PetStoreClient`](struct.PetStoreClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
pub mod model;
pub mod request_model;

use crate::model::*;

pub struct PetStoreClient {
    pub(crate) client: httpclient::Client,
}
impl PetStoreClient {}
impl PetStoreClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        PetStoreClient { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///List all pets
    pub fn list_pets(&self) -> request_model::ListPetsRequest {
        request_model::ListPetsRequest {
            client: &self,
            limit: None,
        }
    }
    ///Info for a specific pet
    pub fn show_pet_by_id(&self, pet_id: String) -> request_model::ShowPetByIdRequest {
        request_model::ShowPetByIdRequest {
            client: &self,
            pet_id,
        }
    }
}

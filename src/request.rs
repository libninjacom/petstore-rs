use serde_json::json;
use crate::model::*;
use crate::PetstoreClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListPetsRequest<'a> {
    pub(crate) client: &'a PetstoreClient,
    pub limit: Option<i64>,
}
impl<'a> ListPetsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Pets> {
        let mut r = self.client.client.get("/pets");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreatePetsRequest<'a> {
    pub(crate) client: &'a PetstoreClient,
}
impl<'a> CreatePetsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/pets");
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ShowPetByIdRequest<'a> {
    pub(crate) client: &'a PetstoreClient,
    pub pet_id: i64,
}
impl<'a> ShowPetByIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Pet> {
        let mut r = self
            .client
            .client
            .get(&format!("/pets/{pet_id}", pet_id = self.pet_id));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}

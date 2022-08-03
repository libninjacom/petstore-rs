use serde::{Serialize, Deserialize};
use crate::model;
pub struct ListPetsRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub limit: Option<i64>,
}
impl<'a> ListPetsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Pets> {
        let limit = self.limit;
        let mut r = self.client.get(&format!("/pets"));
        if let Some(limit) = self.limit {
            r = r.push_query("limit", &limit.to_string());
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
pub struct ShowPetByIdRequest<'a> {
    pub(crate) client: &'a httpclient::Client,
    pub pet_id: String,
}
impl<'a> ShowPetByIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Pet> {
        let pet_id = self.pet_id;
        let mut r = self.client.get(&format!("/pets/{pet_id}"));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}

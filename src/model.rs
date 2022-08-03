use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub tag: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pets(pub Vec<Pet>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

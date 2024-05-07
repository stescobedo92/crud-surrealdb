use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct Product {
    pub id: Option<Thing>,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub quantity: u32
}
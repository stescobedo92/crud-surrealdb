use surrealdb::engine::remote::ws::Client;
use surrealdb::{Error, Surreal};
use surrealdb::error::Db::Thrown;
use surrealdb::method::Content;
use crate::db_context::get_database;
use crate::model::Product;

pub struct ProductRepository {
    table: String,
    db: Surreal<Client>
}

impl ProductRepository {
    pub async fn new() -> Self {
        let db = get_database().await.unwrap();

        ProductRepository{
            table: String::from("product"),
            db
        }
    }
    pub async fn get_all(&self) -> Result<Vec<Product>, Error> {
        let records = self.db.select(&self.table).await?;

        Ok(records)
    }
    pub async fn get_by_id(&self, id: String) -> Result<Product, Error> {
        if let Some(record) = self.db.select((&self.table, id.clone())).await? {
            return Ok(record);
        }

        let error = Error::Db(
            Thrown(
                format!("Product with id {} not found", id)
            )
        );
        Err(error)
    }
    pub async fn create_product(&self, content: Product) -> Result<Vec<Product>, Error> {
        let records = self.db.create(&self.table).content(content).await?;

        Ok(records)
    }
    pub async fn update_product(&self, id: String, content: Product) -> Result<Product, Error> {
        let record = self.db.update((&self.table, id)).content(content).await?.unwrap();

        Ok(record)
    }
    pub async fn delete_product(&self, id: String) -> Result<Product, Error> {
        let record = self.db.delete((&self.table, id)).await?.unwrap();

        Ok(record)
    }
}
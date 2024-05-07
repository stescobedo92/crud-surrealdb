use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Surreal, Result};

pub async fn get_database() -> Result<Surreal<Client>> {
    let db: Surreal<Client> = Surreal::init();
    let _ = db.connect::<Ws>("localhost:8000").await?;
    let _ = db.use_ns("products").use_db("products").await?;

    Ok(db)
}
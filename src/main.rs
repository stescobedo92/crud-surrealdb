use crud_surrealdb::model::Product;
use crud_surrealdb::product_repository::ProductRepository;

#[tokio::main]
async fn main() {
    let repository = ProductRepository::new().await;

    let product = Product {
        id: None,
        name: "Shoes102".to_string(),
        description: "This is the sports shoes".to_string(),
        price: 15.0,
        quantity: 8000
    };

    let result = repository.create_product(product).await.unwrap();
    //let result = repository.get_by_id(String::from("ID_GENERATED_AFTER_CREATE")).await.unwrap();
    //let result = repository.update_product(String::from("ID_GENERATED_AFTER_CREATE"), product).await.unwrap();

    println!("{:?}", result);
}

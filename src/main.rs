use crud_surrealdb::model::Product;
use crud_surrealdb::product_repository::ProductRepository;

#[tokio::main]
async fn main() {
    let repository = ProductRepository::new().await;

    let product = Product {
        id: None,
        name: "Shoes101".to_string(),
        description: "This is the sports shoes".to_string(),
        price: 10.0,
        quantity: 9000
    };

    let result = repository.create_product(product).await.unwrap();

    println!("{:?}", result);
}

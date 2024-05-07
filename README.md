# crud-surrealdb

This program demonstrates basic CRUD (Create, Read, Update, Delete) operations using SurrealDB for managing product data.

## Prerequisites

- Rust programming language installed.
- Async runtime (Tokio) in the dependencies.

## Setup

To use the SurrealDB CRUD operations, follow these steps:

1. Clone the `crud_surrealdb` repository.
2. Add the SurrealDB dependency to your `Cargo.toml`.

```toml
[dependencies]
surrealdb = "1.4.2"
serde = "1.0.200"
tokio = { version = "1.37.0", features = ["full"] }
```
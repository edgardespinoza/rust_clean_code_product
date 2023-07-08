use crate::domain::{ repository::ProductRepository, model::product::Product};
use async_trait::async_trait;
use error::AppError;
use sqlx::PgPool;
use tracing::info;

use super::model::ProductEntity;

#[derive(Clone)]
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresRepository { pool }
    }
}

#[async_trait]
impl ProductRepository for PostgresRepository {
    async fn get_product(&self) -> Result<Vec<Product>, AppError> {
        let entities = sqlx::query_as::<_, ProductEntity>("SELECT * FROM product")
            .fetch_all(&self.pool)
            .await?;

        let products: Vec<Product> = entities.into_iter().map(Product::from).collect();

        Ok(products)
    }

    async fn add_product(&self, product: Product) -> Result<(), AppError> {
        let result =
            sqlx::query("INSERT INTO product (id, name, price, stock) VALUES ($1, $2, $3, $4)")
                .bind(product.id())
                .bind(product.name())
                .bind(product.price())
                .bind(product.stock())
                .execute(&self.pool)
                .await?;

        info!("success commit output: {result:?}");

        Ok(())
    }
}

#[cfg(test)]
mod test_super {
    /*
    use std::result;

    use config::{create_pool, Config};

    use crate::domain::{repository::ProductRepository, product::Product};

    use super::PostgresRepository;

    #[sqlx::test]
    fn test_get_products() {
        let config = Config::from_env();
        let database = create_pool(config.database_url(), config.max_connection)
            .await
            .unwrap();
        let repository = PostgresRepository::new(database);
        let result = repository.get_product().await;

        match result {
            Ok(items) => println!("Ok {:?}", items),
            Err(error) => println!("error {:?}", error),
        }
    }

    #[sqlx::test]
    fn test_add_product(){
        let config = Config::from_env();
        let database = create_pool(config.database_url(), config.max_connection)
            .await
            .unwrap();
        let repository = PostgresRepository::new(database);

        let product = Product::new(12, String::from("name"), 10.).unwrap();

        let result = repository.add_product(product).await;

        match result{
            Ok(_) => println!("Ok"),
            Err(error) => println!("error {:?}", error),

        }
    }*/
}

use async_trait::async_trait;
use error::AppError;

use super::model::product::Product;

#[async_trait]
pub trait ProductRepository {
    async fn get_product(&self) -> Result<Vec<Product>, AppError>;
    async fn add_product(&self, product: Product) -> Result<(), AppError>;
}

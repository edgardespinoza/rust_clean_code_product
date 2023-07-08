use error::AppError;

use crate::domain::{repository::ProductRepository, model::product::Product};


pub struct AddProductUseCase {
     repository: Box<dyn ProductRepository>,
}

impl AddProductUseCase {
    pub fn new(repository: Box<dyn ProductRepository>) -> AddProductUseCase {
        AddProductUseCase { repository }
    }

    pub async fn add_product(&self, product: Product) ->  Result<(), AppError> {
        self.repository.add_product(product).await
    }
}
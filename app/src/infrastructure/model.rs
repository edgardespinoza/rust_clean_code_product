use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::model::product::Product;


#[derive(Serialize, FromRow)]
pub struct ProductEntity {
     id: Uuid,
     name: String,
     price: f32,
     stock: i32,
     image: String
}

impl From<ProductEntity> for Product{
    fn from(item: ProductEntity) -> Self {
        Product::new(  Some(item.id), 
              item.name, item.price, item.stock ).unwrap()
    }
}

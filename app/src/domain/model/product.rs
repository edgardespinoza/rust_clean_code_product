use error::AppError;
use uuid::Uuid;

use super::value_object::ProductName;
#[derive(Debug)]
pub struct Product {
    id: Uuid,
    name: ProductName,
    price: f32,
    stock: i32,
}

impl Product {
    pub fn new(
        id: Option<Uuid>,
        name: String,
        price: f32,
        stock: i32,
    ) -> Result<Product, AppError> {
        let product = Product {
            id: id.unwrap_or_else(Uuid::new_v4),
            name: ProductName::new(name)?,
            price,
            stock,
        };

        Ok(product)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.get()
    }

    pub fn price(&self) -> f32 {
        self.price
    }

    pub fn stock(&self) -> i32 {
        self.stock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_product() {
        let product = Product::new(Some(Uuid::new_v4()), String::from("name"), 1.0, 1);

        assert_eq!(product.unwrap().name(), "name")
    }

    #[test]
    fn test_error_create_product() {
        let result = Product::new(Some(Uuid::new_v4()), String::from(""), 1.0, 1);

        assert!(result.is_err(), "A panic should occur");
    }
}

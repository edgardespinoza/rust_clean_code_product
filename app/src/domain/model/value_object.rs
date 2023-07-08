use error::AppError;
#[derive(Debug)]
pub struct ProductName {
    name: String,
}

impl ProductName {
    pub fn new(name: String) -> Result<ProductName, AppError> {
        ProductName::validate(&name)?;

        Ok(Self { name })
    }

    fn validate(name: &String) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::InvalidData("Name is empty".into()));
        }
        Ok(())
    }

    pub fn get(&self) -> String {
        self.name.clone()
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_name_is_empty() {
        
        let result = ProductName::new(String::from(""));
        
        assert!(result.is_err(), "A panic should occur")
    }

    #[test]
    fn test_name_is_not_empty() {
        
        let name = String::from("Bob");

        let result = ProductName::new(name.clone());

        assert!(result.is_ok(), "Product name creation failed");

        assert_eq!(result.unwrap().get(), name)
    }

}

use crate::{
    infrastructure::postgres_repository::PostgresRepository, domain::use_case::add_product::AddProductUseCase,
};

pub struct WebState {
     pub add_product_use_case: AddProductUseCase,
}
pub fn state_factory(pool: sqlx::PgPool) -> WebState {
    let repository = Box::new(PostgresRepository::new(pool));
    let add_product_use_case = AddProductUseCase::new(repository);
     WebState {
        add_product_use_case,
    } 
    
}

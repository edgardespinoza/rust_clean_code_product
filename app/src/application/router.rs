use actix_web::web;
use super::add_product;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service( add_product::add) 
        );
}
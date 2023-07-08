use actix_web::{post, web, HttpResponse};
use error::AppError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{state::WebState, domain::model::product::Product};


#[utoipa::path(
    post,
    request_body= ProductRequest,
    path = "/api/product",
    responses(
        (status = 201, description = "add product successfully", body = ProductRequest),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError]),

    )
)]
#[post("/product")]
pub async fn add(
    request: web::Json<ProductRequest>,
    webstate: web::Data<WebState>,
) -> Result<HttpResponse, AppError> {
    let result = request.validate();

    if result.is_err() {
        return Err(AppError::InvalidData(result.unwrap_err().to_string()));
    }

    webstate.add_product_use_case.add_product(request.into_inner().into()).await?;

    Ok(HttpResponse::Created().finish())
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct ProductRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Field must be between 1 and 100 characters long"
    ))]
    name: String,

    #[validate(range(min = 1, max = 100, message = "Field must be between 1 and 100"))]
    price: f32,

    stock: i32,
}

impl From<ProductRequest> for Product {
    fn from(item: ProductRequest) -> Self {
        Product::new(None, item.name, item.price, item.stock).unwrap()
    }
}

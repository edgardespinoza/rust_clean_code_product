use once_cell::sync::Lazy;
use utoipa::{
  openapi::security::{Http, HttpAuthScheme, SecurityScheme}, OpenApi, Modify,
  
};

use super::add_product;

use error::{AppError, AppResponseError};

pub static API_DOC: Lazy<utoipa::openapi::OpenApi> = Lazy::new(ApiDoc::openapi);

#[derive(OpenApi)]
#[openapi(
    info(
        version = "v0.1.0",
        title = "RUSTful APIs",
    ),
    paths(
        add_product::add
    ),
    components(
        schemas(
            AppResponseError,
            AppError,
        )
    ),
    tags(
        (name = "product", description = "product endpoints."),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let components = openapi.components.as_mut().unwrap();
    components.add_security_scheme(
      "jwt",
      SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
    )
  }
}

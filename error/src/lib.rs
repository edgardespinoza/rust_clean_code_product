use actix_web::{middleware::ErrorHandlerResponse, dev, Result, http::{header::{self, TryIntoHeaderValue}, StatusCode}, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use actix_web::ResponseError;
use derive_more::{Display};

#[derive(Debug, Display, Serialize, Deserialize, ToSchema)]
#[display(fmt = "my error: {}", error)]
pub struct AppResponseError {
   error: String,
   detail: String
}


#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError{
  #[error("{0}")]
  InvalidData(String),

  #[error("{0}")]
  DatabaseError(String),

}
impl AppError {
  pub fn response(&self) -> AppResponseError {
    AppResponseError {
      error: self.error().to_string(),
      detail: self.to_string(),
    }
  }

  pub fn error(&self) -> &'static str {
    match self {
      Self::InvalidData(_) => "INVALID_INPUT",
      Self::DatabaseError(_) => "DATABASE_ERROR",
    }
}

}
impl  ResponseError for  AppError{
   
  fn error_response(&self) -> HttpResponse {
      HttpResponse::build(self.status_code())
          .json(self.response())
  }

  fn status_code(&self) -> StatusCode {
      match *self {
          AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
          AppError::InvalidData(_) => StatusCode::BAD_REQUEST,
      }
  }
}

impl From<sqlx::Error> for AppError{
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(format!("Database Error: {}", err))
    }
}

pub fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>>
{
     res.response_mut().headers_mut().insert(
         header::CONTENT_TYPE,
         mime::APPLICATION_JSON.try_into_value().unwrap(),
        );

        let (req, res) = res.into_parts();

        let message = res.error().as_ref().map(|error| error.to_string()).unwrap_or_else(|| String::from("No error occurred"));

        let error_response = AppResponseError {
          error: message,
          detail: String::from("")
        };
     
        let res = res.set_body(serde_json::to_string(&error_response)?).map_into_boxed_body();

        let res = dev::ServiceResponse::new(req, res).map_into_right_body();

        Ok(ErrorHandlerResponse::Response(res))
}
 


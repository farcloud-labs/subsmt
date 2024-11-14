#[allow(unused_imports)]
use actix_web::{
    cookie::time::util::weeks_in_year, get, post, web, App, HttpResponse, HttpServer, Responder,
    ResponseError,
};
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Unexpected error occurred")]
    UnexpectedError,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            Error::InternalError(e) => {
                HttpResponse::BadRequest().body(e.to_string())
            }
            _ => {
                HttpResponse::BadRequest().body("Unexpected error occurred")
            }
        }
    }
}

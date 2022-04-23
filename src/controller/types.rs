//! Response types for controllers.

use rocket::{serde::json::Json, Responder};
use serde::Serialize;

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorResponse>),
    #[response(status = 500, content_type = "json")]
    InternalServiceError(Json<ErrorResponse>),
    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorResponse>),
}

#[derive(Debug, Serialize, Responder)]
pub struct ErrorResponse {
    pub message: String,
}

pub type Result<T> = std::result::Result<Json<T>, Error>;

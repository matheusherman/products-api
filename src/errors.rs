// errors.rs
use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest(String),
    Internal,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Not found"),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Internal => write!(f, "Internal server error"),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().body("Not found"),
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().body(msg.clone()),
            ApiError::Internal => HttpResponse::InternalServerError().body("Internal server error"),
        }
    }
}

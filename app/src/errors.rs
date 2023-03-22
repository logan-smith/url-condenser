//! Custom errors (ApiError)

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::Display;
use log::*;

#[derive(Debug, Display, PartialEq, Eq)]
pub enum ApiError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    ParseError(String),
    PoolError(String),
    ReqwestError(String),
    SerdeJsonError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> ApiError {
        error!("Reqwest Error {:?}", error);
        ApiError::ReqwestError(error.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> ApiError {
        error!("Serde Json Error {:?}", error);
        ApiError::SerdeJsonError(error.to_string())
    }
}

/// Converts custom ApiError into Axum acceptable response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(error) => (StatusCode::BAD_REQUEST, error).into_response(),
            ApiError::NotFound(error) => (StatusCode::NOT_FOUND, error).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
        }
    }
}

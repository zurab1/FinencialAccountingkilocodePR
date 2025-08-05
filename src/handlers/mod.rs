pub mod accounts;
pub mod transactions;
pub mod reports;
pub mod web;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// Custom error type for API responses
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message
        }));

        (self.status, body).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Database error: {}", err),
        }
    }
}

// Helper function to create validation errors
pub fn validation_error(message: &str) -> ApiError {
    ApiError {
        status: StatusCode::BAD_REQUEST,
        message: message.to_string(),
    }
}

// Helper function to create not found errors
pub fn not_found_error(resource: &str) -> ApiError {
    ApiError {
        status: StatusCode::NOT_FOUND,
        message: format!("{} not found", resource),
    }
}
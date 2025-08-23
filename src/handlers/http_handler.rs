use axum::{response::{Response, IntoResponse}, http::StatusCode};
use std::net::SocketAddr;

enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<user_routes::LoginModel>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(_data) => (StatusCode::OK).into_response(),
        }
    }
}

enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorized,
    InternalServerError
}
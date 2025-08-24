use axum::{response::{Response, IntoResponse}, http::StatusCode, Json};
use std::net::SocketAddr;
use serde::Serialize;

pub enum ApiResponse<T>
where
    T: Serialize,
{
    Ok,
    Created,
    JsonData(T),
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::Ok => StatusCode::OK.into_response(),
            Self::Created => StatusCode::CREATED.into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

pub enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorized,
    InternalServerError
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub message : String
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody { message: "bad request".into() }),
            ).into_response(),
            ApiError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorBody { message: "forbidden".into() }),
            ).into_response(),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorBody { message: "unauthorized".into() }),
            ).into_response(),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody { message: "internal server error".into() }),
            ).into_response(),
        }
    }
}
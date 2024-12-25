use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub detail: String,
}

pub struct AppError(pub StatusCode, pub Json<ErrorResponse>);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self {
        AppError(
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                detail: err.to_string(),
            }),
        )
    }
}

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    LoginFail,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_ERROR").into_response()
    }
}

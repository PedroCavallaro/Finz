use super::{auth_service::AuthService, dtos::authenticate_dto::AuthenticateDto};
use axum::{http::StatusCode, routing::post, Json, Router};
use tower_cookies::Cookies;

pub struct AuthController {}

impl AuthController {
    async fn authenticate(
        cookies: Cookies,
        Json(authenticate_dto): Json<AuthenticateDto>,
    ) -> Result<(StatusCode, String), StatusCode> {
        match AuthService::authenticate(authenticate_dto).await {
            Ok(token) => Ok((StatusCode::OK, String::from(token))),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    }

    pub fn routes() -> Router {
        Router::new().route("/auth", post(Self::authenticate))
    }
}

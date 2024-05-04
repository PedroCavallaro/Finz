use axum::http::StatusCode;

use super::{auth_repository::AuthRepository, dtos::authenticate_dto::AuthenticateDto};

pub struct AuthService {}

impl AuthService {
    pub async fn authenticate(
        authenticate_dto: AuthenticateDto,
    ) -> Result<&'static str, StatusCode> {
        let account = AuthRepository::find_account(authenticate_dto).await;

        match account.is_some() {
            true => Ok("token"),
            false => Err(StatusCode::NOT_FOUND),
        }
    }
}

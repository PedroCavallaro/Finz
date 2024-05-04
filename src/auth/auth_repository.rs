use sqlx::postgres::PgRow;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::POOL;

use super::dtos::authenticate_dto::AuthenticateDto;
use super::model::{account::Account, user::User};

pub struct AuthRepository {}

impl AuthRepository {
    pub async fn find_account(authenticate_dto: AuthenticateDto) -> Option<PgRow> {
        sqlx::query_as!(
            Account,
            "INSERT INTO Account (id,  provider_id)
             VALUES ($1,'seu_provider_id');",
            Uuid::now_v7(),
        )
        .fetch_one(POOL.get().unwrap())
        .await
        .ok()
    }

    pub fn create_account(authenticate_dto: AuthenticateDto) {
        todo!()
    }
}

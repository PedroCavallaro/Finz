use sqlx::PgPool;

use super::dtos::authenticate_dto::AuthenticateDto;

struct AuthRepository {
    pool: PgPool,
}
impl AuthRepository {
    pub fn find_user(authenticate_dto: AuthenticateDto) {
        todo!()
    }

    pub fn create_user(authenticate_dto: AuthenticateDto) {
        todo!()
    }
}

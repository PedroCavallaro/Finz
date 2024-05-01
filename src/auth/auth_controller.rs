use axum::Router;

use super::auth_service::AuthService;

struct AuthController {
    service: AuthService,
    routes: String,
}

impl AuthController {}

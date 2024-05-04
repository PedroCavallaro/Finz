mod auth;

use std::{env, net::SocketAddr};

use auth::auth_controller::AuthController;
use axum::{middleware, response::Response, routing::get, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::OnceCell;
use tower_cookies::CookieManagerLayer;

pub static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9999);

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("postgres://rinha:rinha@localhost:5432/rinha"));

    let _ = POOL
        .get_or_try_init(|| async {
            PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
        })
        .await;

    let app = Router::new()
        .route("/health-check", get(|| async { "Hello, World!" }))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .merge(AuthController::routes());

    let listener = tokio::net::TcpListener::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    res
}

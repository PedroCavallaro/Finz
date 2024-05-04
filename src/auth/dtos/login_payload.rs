use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginPayload {
    token: String,
}

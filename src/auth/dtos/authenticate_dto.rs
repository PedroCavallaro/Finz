use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AuthenticateDto {
    name: String,
}

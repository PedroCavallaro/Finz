use uuid::Uuid;

pub struct User {
    id: Uuid,
    name: String,
    email: String,
    image_url: String,
}

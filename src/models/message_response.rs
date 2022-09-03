use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse<T> {
    pub code: u32,
    pub content: Option<T>,
}
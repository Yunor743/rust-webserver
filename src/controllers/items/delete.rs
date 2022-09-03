use crate::models;
use crate::database::{DATABASE};
use rocket::get;
use rocket::serde::{json::Json};

#[get("/delete/<id>", format = "application/json")]
pub fn delete(id: usize) -> Json<models::MessageResponse<String>> {
    unsafe { // I use unsafe just for database interaction
        if id >= DATABASE.len() {
            return models::MessageResponse {code: 400, content: Some("Wrong id".to_string()),}.into();
        }
        DATABASE.remove(id);
    }
    models::MessageResponse {code: 200, content: None}.into()
}

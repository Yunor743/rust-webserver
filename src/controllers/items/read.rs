use crate::models;
use crate::database::{DATABASE};
use rocket::get;
use rocket::serde::{json::Json};

#[get("/read/<id>", format = "application/json")]
pub fn read(id: usize) -> Json<models::MessageResponse<models::Item>> {
    unsafe { // I use unsafe just for database interaction
        if id >= DATABASE.len() {
            return models::MessageResponse {code: 400, content: None}.into();
        }
        models::MessageResponse {code: 200, content: Some(DATABASE[id].clone())}.into()
    }
}
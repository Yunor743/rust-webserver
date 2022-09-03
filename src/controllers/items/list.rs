use crate::models;
use crate::database::{DATABASE};
use rocket::get;
use rocket::serde::{json::Json};

#[get("/list", format = "application/json")]
pub fn list() -> Json<models::MessageResponse<&'static [models::Item]>> {
    unsafe { // I use unsafe just for database interaction
        models::MessageResponse {code: 200, content: Some(&DATABASE[..])}.into()
    }
}
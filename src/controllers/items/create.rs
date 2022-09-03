use crate::models;
use crate::database::{DATABASE};
use rocket::post;
use rocket::serde::{json::Json};

#[post("/create", format = "application/json", data = "<message>")]
pub fn create(message: Json<models::Item>) -> Json<models::MessageResponse<String>> {
    let deserialized: models::Item = message.into_inner();
    let value = models::item::new(&deserialized.title, deserialized.number, deserialized.price);
    unsafe { // I use unsafe just for database interaction
        DATABASE.push(value);
    }
    models::MessageResponse {code: 200,content: None,}.into()
}

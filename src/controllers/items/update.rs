use crate::models;
use crate::database::{DATABASE};
use rocket::put;
use rocket::serde::{json::Json};

#[put("/update/<id>", format = "application/json", data = "<message>")]
pub fn update(id: usize, message: Json<models::Item>) -> Json<models::MessageResponse<String>> {
    let deserialized: models::Item = message.into_inner();
    let value = models::item::new(&deserialized.title, deserialized.number, deserialized.price);
    unsafe { // I use unsafe just for database interaction
        if id >= DATABASE.len() {
            return models::MessageResponse {code: 400, content: Some("Wrong id".to_string())}.into();
        }
        DATABASE[id] = value
    }
    models::MessageResponse {code: 200, content: None,}.into()
}

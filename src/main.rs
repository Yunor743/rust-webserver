mod views;
mod controllers;
mod models;
mod database;

use crate::database::{DATABASE};
use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    // Init database with fake values
    let milk = models::item::new(&"Milk".to_string(), 2, 3);
    let eggs = models::item::new(&"Eggs".to_string(), 12, 4);
    unsafe { // I use unsafe just for database interaction
        DATABASE.push(milk);
        DATABASE.push(eggs);
    }

    // Run the server
    rocket::build()
    .mount("/api/items/", routes![
        controllers::items::Create,
        controllers::items::Read,
        controllers::items::Update,
        controllers::items::Delete,
        controllers::items::List
        ])
    .mount("/", routes![views::Index])
}

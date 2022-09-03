use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub title: String,
    pub number: u32,
    pub price: u32,
}

pub fn new(title: &String, number: u32, price: u32) -> Item {
    Item { title: title.to_string(), number: number, price: price }
}
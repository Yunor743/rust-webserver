use rocket::get;
use std::fs;
use rocket::response::content;

#[get("/")]
pub fn index() -> content::Html<String> {
    let content = fs::read_to_string("static/index.html").expect("Unable to read file");
    content::Html(content)
}

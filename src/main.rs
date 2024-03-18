mod cors;
mod db;
mod message;


use tokio;
#[macro_use] extern crate rocket;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[get("/")]
fn index() -> String {
    return "Howdy".to_string()
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

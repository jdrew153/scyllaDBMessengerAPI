mod cors;
mod db;
mod message;


use rocket::serde::json::Json;
use scylla::{Session, SessionBuilder};
use tokio;
use crate::cors::CORS;
use crate::message::message::{read_messages, ReturnMessageType};

#[macro_use] extern crate rocket;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn create_session(uri: &str) -> Result<Session> {
    SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .map_err(From::from)
}

#[get("/all")]
pub async fn read_all_messages() -> Json<Vec<ReturnMessageType>> {
    let uri = "viaduct.proxy.rlwy.net:19973";
    let session = create_session(uri).await.unwrap();

    let messages = read_messages(&session).await;

    match messages {
        Ok(message) => Json(message),
        Err(e) => Json(Vec::new())
    }
}

#[get("/")]
fn index() -> String {
    return "Howdy".to_string()
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, read_all_messages])
}

mod cors;
mod db;
mod message;


use std::result;
use rocket::http::Status;
use rocket::serde::json::Json;
use scylla::{Session, SessionBuilder};
use tokio;
use crate::cors::CORS;
use crate::message::message::{add_message, delete_message, Message, read_messages, ReturnMessageType};

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

#[post("/create-message", data="<body>")]
async fn create_message(body: Json<ReturnMessageType>) -> Status {
    let uri = "viaduct.proxy.rlwy.net:19973";
    let session = create_session(uri).await.unwrap();

    let message: Message = Message{
        id: body.id.clone(),
        time: body.time,
        body:body.body.clone(),
        username: body.username.clone(),
        write_time: body.write_time.clone()
    };

   add_message(&session, &message).await.unwrap();


    return Status::Created
}

#[delete("/delete-message/<id>/<username>")]
async fn delete_message_handler(id: String, username:String) -> Status {

    let uri = "viaduct.proxy.rlwy.net:19973";
    let session = create_session(uri).await.unwrap();

    delete_message(&session, id, username).await;

    return Status::Ok;
}

#[get("/")]
fn index() -> String {
    return "Howdy".to_string()
}
#[launch]
fn rocket() -> _ {
    let envPort = std::env::var("PORT").unwrap();
    rocket::build()
        .attach(CORS)
        .configure(rocket::Config::figment().merge(("port", envPort.parse::<i16>().unwrap())))
        .mount("/", routes![index, read_all_messages, create_message, delete_message_handler ])
}

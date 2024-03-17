use chrono::Utc;
use futures::StreamExt;
use scylla::{FromRow, SerializeRow, Session, ValueList};
use rocket::serde::{Serialize, Deserialize};


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, FromRow, ValueList, SerializeRow, Serialize)]
pub struct Message {
    pub id: String,
    pub time: i64,
    pub body: String,
    pub username: String,
    pub write_time: String,
}

#[derive(Debug, FromRow, ValueList, SerializeRow, Serialize, Deserialize)]
pub struct ReturnMessageType {
    pub id: String,
    pub username: String,
    pub body: String,
    pub time: i64,
    pub write_time: String,
}

static ADD_MESSAGE: &str = r#"INSERT INTO messages.message (id, time, body, username, write_time) VALUES (?, ?, ?, ?, ?)"#;
static SELECT_ALL_MESSAGES: &str = r#"SELECT * FROM messages.message"#;
static DELETE_MESSAGE: &str = r#"DELETE FROM messages.message WHERE id = ? AND username = ?"#;

static UPDATE_MESSAGE: &str = r#"UPDATE messages.message SET write_time = ? WHERE id = ? AND username = ?"#;

pub async fn add_message(session: &Session, message: &Message) -> Result<()> {
    let now = Utc::now().timestamp_millis();
    let result = session
        .query(ADD_MESSAGE, message)
        .await;

    match result {
        Ok(_) => {

            let delta =  Utc::now().timestamp_millis() - now;

            return update_message(session, message.id.clone(), message.username.clone(), delta).await;
        }
        Err(e) => {
            println!("Error: {:?}", e);

            Err(From::from(e))
        }
    }
}

async fn update_message(session: &Session, id: String, username: String, time: i64) -> Result<()> {
    session
        .query(UPDATE_MESSAGE, (time.to_string(), id, username))
        .await
        .map(|_| ())
        .map_err(From::from)
}

pub async fn delete_message(session: &Session, id: String, username: String) -> Result<()> {
    session
        .query(DELETE_MESSAGE, (id, username))
        .await
        .map(|_| ())
        .map_err(From::from)
}

pub async fn read_messages(session: &Session) -> Result<Vec<ReturnMessageType>> {
    let result = session.query(SELECT_ALL_MESSAGES, &[]).await?;

    let mut messages: Vec<ReturnMessageType> = Vec::new();

    let mut iter = result.rows_typed::<ReturnMessageType>()?;

    while let Some(row) = iter.next() {
        let message = row?;
        messages.push(message);
    }


    Ok(messages)
}
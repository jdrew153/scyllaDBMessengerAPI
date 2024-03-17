use scylla::Session;
static CREATE_MESSAGES_KEYSPACE: &str = r#"
    CREATE KEYSPACE IF NOT EXISTS messages
    WITH REPLICATION = {
      'class': 'SimpleStrategy',
      'replication_factor': 1
    };
"#;


static CREATE_MESSAGES_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS messages.message (
      id TEXT,
      time BIGINT,
      body TEXT,
      username TEXT,
      write_time TEXT,

      PRIMARY KEY (id, username)
    );
"#;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn init_db(session: &Session) -> Result<()> {
    create_keyspace(session).await?;
    drop_messages_table(session).await?;
    create_message_table(session).await?;
    Ok(())
}

async fn create_keyspace(session: &Session) -> Result<()> {
    session
        .query(CREATE_MESSAGES_KEYSPACE, ())
        .await
        .map(|_| ())
        .map_err(From::from)
}


async fn create_message_table(session: &Session) -> Result<()> {
    session
        .query(CREATE_MESSAGES_TABLE, ())
        .await
        .map(|_| ())
        .map_err(From::from)
}


async fn drop_messages_table(session: &Session) -> Result<()> {
    let query = "DROP TABLE IF EXISTS messages.message";
    session
        .query(query, ())
        .await
        .map(|_| ())
        .map_err(From::from)
}
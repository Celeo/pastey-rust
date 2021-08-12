use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Connection, Executor, SqliteConnection};
use std::{fs::File, path::Path};

const DB_FILE_NAME: &str = "data.db";
const SQL_CREATE_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS pastes (
        uuid TEXT NOT NULL PRIMARY KEY,
        content TEXT,
        lang TEXT,
        add_date DATETIME,
        purge_date DATETIME
    );
";
const SQL_QUERY_ROW: &str = "SELECT * FROM pastes WHERE uuid = ?;";

#[derive(sqlx::FromRow, Serialize)]
pub struct PasteInfo {
    pub uuid: i64,
    pub content: String,
    pub lang: String,
    pub add_date: DateTime<Utc>,
    pub purge_date: DateTime<Utc>,
}

pub async fn get_db() -> std::result::Result<SqliteConnection, sqlx::Error> {
    SqliteConnection::connect(DB_FILE_NAME).await
}

pub async fn setup_db() -> Result<()> {
    if !Path::new(DB_FILE_NAME).exists() {
        File::create(DB_FILE_NAME)?;
    }
    get_db().await?.execute(SQL_CREATE_TABLE).await?;
    Ok(())
}

pub async fn get_paste(uuid: &str) -> Result<Option<PasteInfo>> {
    let mut conn = get_db().await?;
    let info = sqlx::query_as::<_, PasteInfo>(SQL_QUERY_ROW)
        .bind(uuid)
        .fetch_optional(&mut conn)
        .await?;
    Ok(info)
}

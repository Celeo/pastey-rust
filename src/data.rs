use anyhow::Result;
use sqlx::{Connection, Executor, SqliteConnection};
use std::{fs::File, path::Path};

const DB_FILE_NAME: &str = "data.db";
const SQL_CREATE_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS pastes (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        content TEXT,
        lang TEXT,
        update_code TEXT,
        add_date DATETIME,
        purge_date DATETIME
    );
";

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

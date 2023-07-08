/*

    Application Data.

*/

use std::str::FromStr;

use serde::{ Serialize, Deserialize };
use sqlx::{ SqlitePool, FromRow };
use sqlx::sqlite::{
    SqliteConnectOptions,
    SqliteJournalMode,
    SqlitePoolOptions,
    SqliteSynchronous
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Term
{
    pub term_id: i64,
    pub term: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Mask
{
    pub mask_id: i64,
    pub term_id: i64,
    pub mask: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuizResult
{
    pub quiz_result_id: i64,
    pub quiz_cnt: i64,
    pub correct_cnt: i64,
    pub incorrect_cnt: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Answer
{
    pub answer_id: i64,
    pub quiz_result_id: i64,
    pub is_correct: bool,
}

//  Database Result Type.
type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

//------------------------------------------------------------------------------
//  Creates a new database connection pool.
//------------------------------------------------------------------------------
pub async fn create_sqlite_pool( database_url: &str ) -> DbResult<SqlitePool>
{
    let connect_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connect_options)
        .await?;

    Ok(sqlite_pool)
}

//------------------------------------------------------------------------------
//  Migration.
//------------------------------------------------------------------------------
pub async fn migrate_database( pool: &SqlitePool ) -> DbResult<()>
{
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    Ok(())
}

//------------------------------------------------------------------------------
//  Creates a new quiz result.
//------------------------------------------------------------------------------
pub async fn create_quiz_result( pool: &SqlitePool ) -> DbResult<QuizResult>
{
    let sql = r#"
        INSERT INTO
            `quiz_result`
            (`quiz_cnt`, `correct_cnt`, `incorrect_cnt`)
        VALUES
            (0, 0, 0)"#;
    let row = sqlx::query_as::<_, QuizResult>(sql)
        .fetch_one(pool)
        .await?;
    Ok(row)
}

/*

    Tauri endpoint.

*/

#![cfg_attr
(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;
mod command;

use sqlx::SqlitePool;
use tauri::async_runtime::block_on;
use tauri::Manager;

const DATABASE_DIR: &str = "qzgen";
const DATABASE_FILE: &str = "db.sqlite";

pub struct AppState
{
    pub sqlite_pool: SqlitePool,
    pub quiz_result: database::QuizResult,
}

fn main()
{
    let home_dir = directories::UserDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    let database_dir = home_dir.join(DATABASE_DIR);
    let database_file = database_dir.join(DATABASE_FILE);

    //  Creates a database directory if it does not exist.
    let db_exists = std::fs::metadata(&database_file).is_ok();
    if db_exists == false
    {
        std::fs::create_dir(&database_dir).unwrap();
    }

    //  Creates a database connection pool.
    let database_dir_str = dunce::canonicalize(database_dir)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let database_url = format!
    (
        "sqlite://{}/{}",
        database_dir_str,
        DATABASE_FILE,
    );
    let sqlite_pool = block_on(database::create_sqlite_pool(&database_url))
        .expect("error while creating sqlite pool");
    if db_exists == false
    {
        block_on(database::migrate_database(&sqlite_pool))
            .expect("error while creating tables");
    }

    //  Creates a result record.
    let quiz_result = block_on(database::create_quiz_result(&sqlite_pool))
        .expect("error while creating a quiz result");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler!
        [
            command::get_random_term_and_masks,
        ])
        .setup(|app|
        {
            let app_state = AppState
            {
                sqlite_pool: sqlite_pool,
                quiz_result: quiz_result,
            };
            app.manage(app_state);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

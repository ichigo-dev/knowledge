use std::env;
use std::fs::{ self, File };
use std::io::{ Write, BufReader };

use dotenv::dotenv;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::{ Connection, ConnectOptions };
use regex::Regex;
use chrono::Local;
use reqwest::Client;
use zip::ZipArchive;

#[tokio::main]
async fn main()
{
    println!("Start to update terms.");
    dotenv().ok();

    //  Gets current time.
    let now = Local::now();
    let now = now.naive_local();
    let now = now.format("%Y-%m-%d %H:%M:%S").to_string();

    //  Gets repository from GitHub.
    println!("Getting repository...");
    let zip_file_name = env::var("ZIP_FILE_NAME")
        .expect("ZIP_FILE_NAME is not set");
    let repo_name = env::var("REPO_NAME")
        .expect("REPO_NAME is not set");
    download_repository(&zip_file_name).await;
    unzip_file(&zip_file_name, "./").await;
    let _ = fs::remove_file(&zip_file_name);

    //  Reads .env file.
    let note_path = env::var("NOTE_PATH")
        .expect("NOTE_PATH is not set");
    let db_host = env::var("DB_HOST").expect("DB_HOST is not set");
    let db_port = env::var("DB_PORT").expect("DB_PORT is not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME is not set");
    let db_user = env::var("DB_USER").expect("DB_USER is not set");
    let db_pass = env::var("DB_PASS").expect("DB_PASS is not set");

    //  Connects to database.
    println!("Connecting to database...");
    let mut pool = MySqlConnectOptions::new()
        .host(&db_host)
        .port(db_port.parse().unwrap_or(3306))
        .database(&db_name)
        .username(&db_user)
        .password(&db_pass)
        .connect()
        .await
        .expect("failed to connect database");
    let mut tx = pool.begin().await.expect("failed to begin transaction");

    println!("Updating terms...");
    let paths = get_markdown_recursive(&note_path);
    for path in paths
    {
        let content = fs::read_to_string(&path).unwrap();
        let sections = match get_sections_from_markdown(&content)
        {
            Some(s) => s,
            None => continue,
        };

        println!("  {}", path);
        for section in sections
        {
            let terms = get_terms_from_section(&section);
            for term in terms
            {
                update_term(&mut tx, &path, &term, &section, &now).await;
            }
        }
    }
    delete_terms(&mut tx, &now).await;
    tx.commit().await.unwrap();
    let _ = fs::remove_dir_all(&repo_name);
}


//------------------------------------------------------------------------------
//  Downloads repository from GitHub.
//------------------------------------------------------------------------------
async fn download_repository( file_name: &str )
{
    let endpoint = env::var("REPO_ENDPOINT")
        .expect("REPO_ENDPOINT is not set");
    let client = Client::new();
    let response = client
        .get(&endpoint)
        .send()
        .await
        .expect("failed to get repository");
    let body = response.bytes().await.expect("failed to get body");
    let mut out = File::create(file_name)
        .expect("failed to create a zip file");
    out.write_all(body.as_ref())
        .expect("failed to write body");
}


//------------------------------------------------------------------------------
//  Unzips file.
//------------------------------------------------------------------------------
async fn unzip_file( from_name: &str, to_name: &str )
{
    let file = File::open(from_name).expect("failed to open a zip file");
    let reader = BufReader::new(file);
    let mut zip = ZipArchive::new(reader).expect("failed to read a zip file");

    for i in 0..zip.len()
    {
        let mut entry = zip.by_index(i).unwrap();
        let extract_path = format!("{}/{}", to_name, entry.name());
        if entry.is_dir()
        {
            fs::create_dir_all(&extract_path).unwrap();
        }
        else
        {
            let mut out = File::create(&extract_path).unwrap();
            std::io::copy(&mut entry, &mut out).unwrap();
        }
    }
}


//------------------------------------------------------------------------------
//  Gets markdown file recursive.
//------------------------------------------------------------------------------
fn get_markdown_recursive( base_path: &str ) -> Vec<String>
{
    let mut result = Vec::new();
    let mut stack = Vec::new();
    stack.push(base_path.to_string());

    while let Some(base_path) = stack.pop()
    {
        let dir = std::fs::read_dir(base_path).unwrap();
        for entry in dir
        {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = entry.file_name();
            if path.is_dir()
            {
                stack.push(path.to_str().unwrap().to_string());
            }
            else
            {
                if let Some(ext) = path.extension()
                {
                    if ext == "md"
                        && file_name != "README.md"
                        && file_name != "checksheet.md"
                    {
                        result.push(path.to_str().unwrap().to_string());
                    }
                }
            }
        }
    }

    result
}


//------------------------------------------------------------------------------
//  Gets section from markdown file.
//------------------------------------------------------------------------------
fn get_sections_from_markdown( content: &str ) -> Option<Vec<String>>
{
    let mut sections = Vec::new();
    let mut section = String::new();

    //  Gets sections from markdown file.
    for line in content.lines()
    {
        if line.starts_with("#")
        {
            if section.is_empty() == false
            {
                sections.push(section);
            }

            section = String::new();
        }
        else if line.is_empty() == false
        {
            section.push_str(&line);
        }
    }

    if sections.len() <= 2
    {
        return None;
    }
    else
    {
        //  Removes note title and ToC(Table of Contents) section.
        sections = sections.split_off(2);
    }

    Some(sections)
}


//------------------------------------------------------------------------------
//  Gets terms from section.
//------------------------------------------------------------------------------
fn get_terms_from_section( section: &str ) -> Vec<String>
{
    let re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let mut terms = Vec::new();
    for capture in re.captures_iter(section)
    {
        if let Some(term) = capture.get(1)
        {
            if terms.contains(&term.as_str().to_string())
            {
                continue;
            }

            terms.push(term.as_str().to_string());
        }
    }

    terms
}

//------------------------------------------------------------------------------
//  Updates term.
//------------------------------------------------------------------------------
async fn update_term
(
    tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
    path: &str,
    term: &str,
    section: &str,
    now: &str,
)
{
    let exists_term = sqlx::query
    (
        r#"
            SELECT
                `term_id`
            FROM
                `term`
            WHERE
                `path` = ? AND
                `term` = ?
        "#
    )
    .bind(&path)
    .bind(&term)
    .fetch_one(&mut **tx)
    .await;

    if exists_term.is_ok()
    {
        sqlx::query
        (
            r#"
                UPDATE `term`
                SET
                    `content` = ?,
                    `updated_at` = ?
                WHERE
                    `path` = ? AND
                    `term` = ?
            "#
        )
        .bind(&section)
        .bind(&now)
        .bind(&path)
        .bind(&term)
        .execute(&mut **tx)
        .await
        .expect("failed to update term");
    }
    else
    {
        sqlx::query
        (
            r#"
                INSERT INTO `term`
                (`path`, `term`, `content`, `created_at`, `updated_at`)
                VALUES
                (?, ?, ?, ?, ?)
            "#
        )
        .bind(&path)
        .bind(&term)
        .bind(&section)
        .bind(&now)
        .bind(&now)
        .execute(&mut **tx)
        .await
        .expect("failed to insert term");
    }
}

//------------------------------------------------------------------------------
//  Deletes terms.
//------------------------------------------------------------------------------
async fn delete_terms( tx: &mut sqlx::Transaction<'_, sqlx::MySql>, now: &str )
{
    sqlx::query
    (
        r#"
            DELETE FROM
                `term`
            WHERE
                `updated_at` < ?
        "#
    )
    .bind(&now)
    .execute(&mut **tx)
    .await
    .expect("failed to delete terms");
}

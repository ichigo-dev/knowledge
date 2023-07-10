use std::env;
use std::fs;

use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use regex::Regex;

#[tokio::main]
async fn main()
{
    //  Reads .env file.
    dotenv().ok();
    let note_path = env::var("NOTE_PATH")
        .expect("NOTE_PATH is not set");
    let db_host = env::var("DB_HOST").expect("DB_HOST is not set");
    let db_port = env::var("DB_PORT").expect("DB_PORT is not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME is not set");
    let db_user = env::var("DB_USER").expect("DB_USER is not set");
    let db_pass = env::var("DB_PASS").expect("DB_PASS is not set");

    let paths = get_markdown_recursive(&note_path);
    for path in paths
    {
        let content = fs::read_to_string(path).unwrap();
        let sections = match get_sections_from_markdown(&content)
        {
            Some(s) => s,
            None => continue,
        };

        for section in sections
        {
            let terms = get_terms_from_section(&section);

            for term in terms
            {

            }
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

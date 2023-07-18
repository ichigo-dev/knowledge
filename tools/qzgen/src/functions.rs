use regex::{ Regex, Captures };
use reqwest::Client;
use pulldown_cmark::{ html, Parser, Options };
use uuid::Uuid;

use crate::{
    API_URL,
    API_KEY,
    NOTE_URL,
    NOTE_PATH_PREFIX,
    USER_CODE_KEY,
    Term,
    User,
    UserResult,
};


//------------------------------------------------------------------------------
//  Gets user information (create if it doesn't exist).
//------------------------------------------------------------------------------
pub async fn get_or_create_user() -> User
{
    //  Gets user code from local storage.
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("local storage should be available");
    let user_code = if let Ok(Some(user_code))
        = local_storage.get_item(USER_CODE_KEY)
    {
        user_code
    }
    else
    {
        let user_code = Uuid::new_v4().to_string();
        local_storage.set_item(USER_CODE_KEY, &user_code).unwrap();
        user_code
    };

    //  Gets user information.
    let client = Client::new();
    let url = API_URL.to_string() + "/get_or_create_user/" + &user_code;
    let response = client
        .get(url)
        .header("x-api-key", API_KEY)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    let user: User = serde_json::from_str(&body).unwrap();
    user
}


//------------------------------------------------------------------------------
//  Generate quiz.
//
//  Return:
//      (
//          quiz: String,
//          answer: String,
//          answer_url: String,
//      )
//------------------------------------------------------------------------------
pub async fn generate_quiz() -> (String, String, String)
{
    let client = Client::new();
    let url = API_URL.to_string() + "/generate_random_quiz";
    let response = client
        .get(url)
        .header("x-api-key", API_KEY)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    let term: Term = serde_json::from_str(&body).unwrap();

    //  Converts markdown to HTML.
    let content = term.content;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&content.trim(), options);
    let mut html_str = String::new();
    html::push_html(&mut html_str, parser);

    //  Replaces href link.
    let path = term.path;
    let path = path.replace(NOTE_PATH_PREFIX, "");
    let (path_note, _) = path
        .rsplit_once('#')
        .unwrap_or((&path, ""));
    let (path_base, _) = path
        .rsplit_once('/')
        .unwrap_or((&path, ""));
    let re = Regex::new(r#"href="([^"]+)""#).unwrap();
    let html_str = re.replace_all(&html_str, |caps: &Captures|
    {
        let mut url = caps.get(1).unwrap().as_str().to_string();
        if url.starts_with('#') == false
        {
            url = NOTE_URL.to_string() + &path_base + "/" + &url;
        }
        else
        {
            url = NOTE_URL.to_string() + &path_note + &url;
        }
        format!
        (
            "target=\"_blnak\" rel=\"noreferrer noopener\" href=\"{}\"",
            url
        )
    });

    //  Replaces the term with a mask.
    let mask = "<label class=\"mask\" for=\"user_answer\">_____</label>";
    let quiz = html_str.replace(&term.term, mask);

    (quiz, term.term, (NOTE_URL.to_string() + &path))
}


//------------------------------------------------------------------------------
//  Creates user result.
//------------------------------------------------------------------------------
pub async fn create_user_result( user: &User ) -> UserResult
{
    let client = Client::new();
    let url = API_URL.to_string()
        + "/create_user_result/"
        + &user.user_id.to_string();
    let response = client
        .get(url)
        .header("x-api-key", API_KEY)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    let user_result: UserResult = serde_json::from_str(&body).unwrap();
    user_result
}

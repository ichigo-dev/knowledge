/*

    Quiz page.

*/

use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use reqwest::Client;
use pulldown_cmark::{ html, Parser };
use regex::{ Regex, Captures };

use crate::{ API_URL, NOTE_URL, NOTE_PATH_PREFIX, Term };


//------------------------------------------------------------------------------
//  Quiz page component.
//------------------------------------------------------------------------------
#[component]
pub fn Quiz<G: Html>( cx: Scope ) -> View<G>
{
    let quiz = create_signal(cx, String::new());
    let answer = create_signal(cx, String::new());
    let update_quiz = || async
    {
        quiz.set("Generating quiz...".to_string());
        let (new_quiz, new_answer) = generate_quiz().await;
        quiz.set(new_quiz);
        answer.set(new_answer);
    };
    spawn_local_scoped(cx, async move
    {
        update_quiz().await;
    });

    view!
    {
        cx,

        //  Quiz
        div
        (
            class="quiz_panel ui_panel padding_lg margin_bottom_lg",
            dangerously_set_inner_html=&quiz.get()
        )

        //  Answer
        div(class="ui_panel padding_lg margin_bottom_lg")
        {
            label(class="flex column align_start")
            {
                span(class="ui_label") { "Answer" }
                input(type="text", class="ui_text", id="user_answer")
            }
        }

        div(class="spacer")

        //  Operation
        div(class="flex row justify_between")
        {
            a(href="/", class="ui_button error") { "Quit" }

            button(class="ui_button success") { "Answer" }

            button(class="ui_button info") { "Hint" }

            button
            (
                class="ui_button warn",
                on:click=move |_|
                {
                    spawn_local_scoped(cx, async move
                    {
                        update_quiz().await;
                    })
                }
            )
            {
                "Give up"
            }
        }
    }
}


//------------------------------------------------------------------------------
//  Generate quiz.
//------------------------------------------------------------------------------
async fn generate_quiz() -> (String, String)
{
    let client = Client::new();
    let url = API_URL.to_string() + "/generate_random_quiz";
    let response = client.get(url).send().await.unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    let term: Term = serde_json::from_str(&body).unwrap();

    //  Converts markdown to HTML.
    let content = term.content;
    let parser = Parser::new(&content);
    let mut html_str = String::new();
    html::push_html(&mut html_str, parser);

    //  Replaces href link.
    let path = term.path;
    let path = path.replace(NOTE_PATH_PREFIX, "");
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
            url = NOTE_URL.to_string() + &path + &url;
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

    (quiz, term.term)
}

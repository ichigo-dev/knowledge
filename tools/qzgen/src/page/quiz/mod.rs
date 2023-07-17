/*

    Quiz page.

*/

mod quit_popup;
mod giveup_popup;
mod answer_popup;

use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use sycamore::rt::JsCast;
use web_sys::{ Event, Element };
use reqwest::Client;
use pulldown_cmark::{ html, Parser, Options };
use regex::{ Regex, Captures };

use quit_popup::QuitPopup;
use giveup_popup::GiveupPopup;
use answer_popup::AnswerPopup;
use crate::{ API_URL, NOTE_URL, NOTE_PATH_PREFIX, MAX_TRY_CNT, Term };


//------------------------------------------------------------------------------
//  Quiz page component.
//------------------------------------------------------------------------------
#[component]
pub fn Quiz<G: Html>( cx: Scope ) -> View<G>
{
    //  Signals.
    let quiz = create_signal(cx, String::new());
    let answer = create_signal(cx, String::new());
    let hint_len = create_signal(cx, 0);
    let hint = create_signal(cx, String::new());
    let control_next = create_signal(cx, false);

    //  User answer.
    let input_answer = create_signal(cx, String::new());
    let remain = create_signal(cx, MAX_TRY_CNT);
    let is_correct = create_signal(cx, false);

    //  Popup states.
    let quit_is_open = create_signal(cx, false);
    let giveup_is_open = create_signal(cx, false);
    let answer_is_open = create_signal(cx, false);

    //  Generates quiz.
    let update_quiz = || async
    {
        //  Resets signals.
        input_answer.set("".to_string());
        hint_len.set(0);
        hint.set("".to_string());
        remain.set(MAX_TRY_CNT);
        is_correct.set(false);

        //  Generates next quiz.
        quiz.set("Generating quiz...".to_string());
        let (new_quiz, new_answer) = generate_quiz().await;
        quiz.set(new_quiz);
        answer.set(new_answer);

        control_next.set(false);
    };

    //  Updates hint.
    let update_hint = ||
    {
        let len = *hint_len.get() + 1;
        let mut new_hint = String::new();
        for (i, char) in answer.get().chars().enumerate()
        {
            if i < len
            {
                new_hint.push(char);
            }
            else
            {
                new_hint.push('_');
            }
        }
        hint.set(new_hint);
        hint_len.set(len);
    };

    //  User answer.
    let try_answer = move ||
    {
        remain.set(*remain.get() - 1);
        if input_answer.get().to_lowercase() == answer.get().to_lowercase()
        {
            is_correct.set(true);
            control_next.set(true);
        }
        else if *remain.get() <= 0
        {
            answer_is_open.set(true);
        }
    };

    //  Initializes quiz.
    spawn_local_scoped(cx, async move
    {
        update_quiz().await;
    });

    view!
    {
        cx,

        //  Quit
        div(class="flex row justify_start margin_bottom_lg")
        {
            button
            (
                class="ui_button error",
                on:click=move |_| { quit_is_open.set(true); }
            ) { "Quit" }
        }

        //  Quiz
        div
        (
            class="quiz_panel ui_panel padding_lg margin_bottom_lg",
            dangerously_set_inner_html=&quiz.get()
        )

        //  Answer
        div(class="ui_panel padding_lg margin_bottom_lg")
        {
            (
                if *is_correct.get()
                {
                    view!
                    {
                        cx,
                        "Great!"
                    }
                }
                else
                {
                    view!
                    {
                        cx,
                        label(class="flex column align_start")
                        {
                            span(class="ui_label")
                            {
                                "Answer (Remain: " (remain.get()) ")"
                            }
                            input
                            (
                                type="text",
                                class="ui_text",
                                id="user_answer",
                                bind:value=input_answer,
                            )
                        }

                        (
                            if hint.get().len() > 0
                            {
                                view!
                                {
                                    cx,
                                    p(class="margin_top")
                                    {
                                        "Hint: " (hint.get())
                                    }
                                }
                            }
                            else
                            {
                                view!{ cx, }
                            }
                        )
                    }
                }
            )
        }

        div(class="spacer")

        //  Operation
        (
            if *control_next.get()
            {
                view!
                {
                    cx,
                    div(class="flex row justify_center")
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |e: Event|
                            {
                                //  Disabled button
                                let target = e.target().unwrap();
                                let elm = target.dyn_ref::<Element>().unwrap();
                                elm.set_class_name("hide");
                                elm.set_attribute("disabled", "true").unwrap();

                                spawn_local_scoped(cx, async move
                                {
                                    update_quiz().await;
                                });
                            },
                        ) { "Next" }
                    }
                }
            }
            else
            {
                view!
                {
                    cx,
                    div(class="flex row justify_between")
                    {
                        button
                        (
                            class="ui_button warn",
                            on:click=move |_| { giveup_is_open.set(true); },
                        ) { "Give up" }

                        button
                        (
                            class="ui_button info",
                            on:click=move |_| { update_hint(); },
                        ) { "Hint" }

                        button
                        (
                            class="ui_button success",
                            on:click=move |_| { try_answer(); },
                        ) { "Answer" }
                    }
                }
            }
        )

        //  Popups
        QuitPopup(is_open=quit_is_open)
        GiveupPopup
        (
            is_open=giveup_is_open,
            callback=Box::new(move ||
            {
                giveup_is_open.set(false);
                answer_is_open.set(true);
            }),
        )
        AnswerPopup
        (
            is_open=answer_is_open,
            answer=answer,
            callback=Box::new(move ||
            {
                answer_is_open.set(false);
                spawn_local_scoped(cx, async move
                {
                    update_quiz().await;
                });
            }),
        )
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
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&content.trim(), options);
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

/*

    Record list.

*/

use sycamore::prelude::*;

use crate::{ AppState, UserResult, UserAnswer };
use crate::component::{ CheckIcon, CrossIcon, EyeIcon };
use crate::functions::{ term_to_quiz, get_user_results };

#[component]
pub async fn RecordList<G: Html>( cx: Scope<'_> ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);
    let api_key = app_state.api_key.get();
    let results = create_signal(cx, Vec::new());
    if let Some(user) = app_state.user.get().as_ref()
    {
        results.set(get_user_results(&api_key, user.user_id).await);
    }

    view!
    {
        cx,
        div(class="flex column gap_md full_horizon")
        {
            Indexed
            (
                iterable = results,
                view = move |cx, result|
                {
                    view!
                    {
                        cx,
                        RecordItem(result=result)
                    }
                },
            )
        }
    }
}

#[component(inline_props)]
fn RecordItem<G: Html>
(
    cx: Scope<'_>,
    result: UserResult,
) -> View<G>
{
    if result.answers.len() <= 0
    {
        return view!{ cx, };
    }

    let answers = create_signal(cx, result.answers);
    let correct_len = answers.get().iter().filter(|a| a.is_correct).count();
    let correct_rate = (correct_len as f32 / answers.get().len() as f32) * 100.0;
    let result_class = if correct_len == answers.get().len()
    {
        "perfect"
    }
    else
    {
        "imperfect"
    };
    view!
    {
        cx,
        div(class="ui_panel padding_lg shadow radius")
        {
            div(class="ui_panel transparent border_bottom padding_bottom_sm margin_bottom_lg flex justify_between")
            {
                span { (result.created_at) }
                span(class=result_class)
                {
                    (correct_len) "/" (answers.get().len()) " "
                    "(" (correct_rate.round() as u32) "%)"
                }
            }
            div(class="flex column align_start gap_lg")
            {
                Indexed
                (
                    iterable = answers,
                    view = move |cx, answer|
                    {
                        view!
                        {
                            cx,
                            RecordAnswer(answer=answer)
                        }
                    },
                )
            }
        }
    }
}

#[component(inline_props)]
fn RecordAnswer<G: Html>
(
    cx: Scope<'_>,
    answer: UserAnswer,
) -> View<G>
{
    let quiz = term_to_quiz(&answer.term);
    let term = create_signal(cx, "*****".to_string());
    let is_close = create_signal(cx, true);
    create_effect(cx, move ||
    {
        if *is_close.get()
        {
            term.set("*****".to_string());
        }
        else
        {
            term.set(answer.term.term.clone());
        }
    });
    view!
    {
        cx,
        div(class="record_answer full_horizon flex column align_start")
        {
            div(class="record_result margin_bottom_md flex justify_between full_horizon")
            {
                div(class="flex align_center gap_md")
                {
                    EyeIcon(is_close=is_close, callback=Some(Box::new
                    (
                        move ||
                        {
                            is_close.set(!(*is_close.get()));
                        }
                    )))
                    (term.get())
                }
                (
                    if answer.is_correct
                    {
                        view!{ cx, CheckIcon }
                    }
                    else
                    {
                        view!{ cx, CrossIcon }
                    }
                )
            }
            div
            (
                class=&("quiz_content ".to_string() + if !(*is_close.get())
                {
                    "show_answer"
                } else { "" }),
                dangerously_set_inner_html=&quiz.quiz
            )
            div(class="record_answer_sep border full_horizon margin_top_md")
        }
    }
}

/*

    Record list.

*/

use sycamore::prelude::*;

use crate::{ AppState, UserResult, UserAnswer };
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
        div(class="flex column gap_md")
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
            div(class="ui_panel transparent border_bottom padding_bottom_sm margin_bottom flex justify_between")
            {
                span { (result.created_at) }
                span(class=result_class)
                {
                    (correct_len) "/" (answers.get().len()) " "
                    "(" (correct_rate.round() as u32) "%)"
                }
            }
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

#[component(inline_props)]
fn RecordAnswer<G: Html>
(
    cx: Scope<'_>,
    answer: UserAnswer,
) -> View<G>
{
    let quiz = term_to_quiz(&answer.term);
    view!
    {
        cx,
        div(class="record_quiz")
        {
            (answer.term.term)
        }
        div
        (
            class="quiz_content",
            dangerously_set_inner_html=&quiz.quiz
        )
        div(class="record_result")
        {
            (
                if answer.is_correct
                {
                    view!{ cx, "O" }
                }
                else
                {
                    view!{ cx, "X" }
                }
            )
        }
    }
}

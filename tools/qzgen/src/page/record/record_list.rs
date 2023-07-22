/*

    Record list.

*/

use sycamore::prelude::*;

use crate::AppState;
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
                    let answers = create_signal(cx, result.answers);
                    view!
                    {
                        cx,
                        Indexed
                        (
                            iterable = answers,
                            view = move |cx, answer|
                            {
                                let quiz = term_to_quiz(&answer.term);
                                view!
                                {
                                    cx,
                                    div(class="ui_panel padding_md")
                                    {
                                        div(class="record_quiz")
                                        {
                                            (answer.term.term)
                                        }
                                        div
                                        (
                                            class="record_answer",
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
                            },
                        )
                    }
                },
            )
        }
    }
}

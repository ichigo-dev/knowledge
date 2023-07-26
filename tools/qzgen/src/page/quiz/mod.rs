/*

    Quiz page.

*/

mod quit_popup;
mod giveup_popup;
mod answer_popup;

use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;

use quit_popup::QuitPopup;
use giveup_popup::GiveupPopup;
use answer_popup::AnswerPopup;

use crate::{ MAX_TRY_CNT, AppState, Quiz, UserResult, UserAnswer };
use crate::component::Loading;
use crate::functions::{
    generate_quiz,
    create_user_result,
    insert_user_answer,
};


//------------------------------------------------------------------------------
//  Quiz page component.
//------------------------------------------------------------------------------
#[component]
pub fn Quiz<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);

    //  Signals.
    let quiz_cnt = create_signal(cx, 1);
    let quiz = create_signal::<Quiz>(cx, Quiz::default());
    let hint_len = create_signal(cx, 0);
    let hint = create_signal(cx, String::new());

    //  User answer.
    let user_result = create_signal(cx, Option::<UserResult>::None);
    let answers = create_signal(cx, Vec::<UserAnswer>::new());
    let input_answer = create_signal(cx, String::new());
    let remain = create_signal(cx, MAX_TRY_CNT);
    let is_correct = create_signal(cx, false);

    //  Popup states.
    let quit_is_open = create_signal(cx, false);
    let giveup_is_open = create_signal(cx, false);
    let answer_is_open = create_signal(cx, false);
    let answer_popup_message = create_signal(cx, view!{ cx, });

    //  Generates quiz.
    let update_quiz = || async
    {
        //  Resets signals.
        quiz.set(Quiz::default());
        input_answer.set("".to_string());
        hint_len.set(0);
        hint.set("".to_string());
        remain.set(MAX_TRY_CNT);

        //  Generates next quiz.
        let api_key = app_state.api_key.get();
        quiz.set(generate_quiz(&api_key).await);
    };

    //  Updates hint.
    let update_hint = ||
    {
        let len = *hint_len.get() + 1;
        let mut new_hint = String::new();
        for (i, char) in quiz.get().as_ref().term.term.chars().enumerate()
        {
            if i < len
            {
                new_hint.push(char);
            }
            else
            {
                new_hint.push('*');
            }
        }

        if len >= quiz.get().as_ref().term.term.chars().count()
        {
            new_hint.push_str(" (No more hints...)");
        }
        else
        {
            hint_len.set(len);
        }
        hint.set(new_hint);
    };

    //  Terminates quiz.
    let terminate_quiz = move ||
    {
        quiz_cnt.set(*quiz_cnt.get() + 1);

        //  Creates user answer.
        if let Some(u) = user_result.get().as_ref()
        {
            let api_key = app_state.api_key.get();
            let user_answer = UserAnswer
            {
                user_answer_id: 0,
                user_result_id: u.user_result_id,
                term_id: quiz.get().as_ref().term.term_id,
                term: quiz.get().as_ref().term.clone(),
                is_correct: *is_correct.get(),
                created_at: "".to_string(),
            };

            spawn_local_scoped(cx, async move
            {
                let user_answer = insert_user_answer
                (
                    &api_key,
                    &user_answer,
                ).await;
                answers.modify().push(user_answer);
            });
        }
    };

    //  User answer.
    let try_answer = move ||
    {
        if input_answer.get().trim().len() <= 0
        {
            return;
        }

        if input_answer.get().to_lowercase().trim()
            == quiz.get().as_ref().term.term.to_lowercase().trim()
        {
            answer_popup_message.set(view!{ cx, "Great!" });
            answer_is_open.set(true);
            is_correct.set(true);
        }

        remain.set(*remain.get() - 1);
        if *remain.get() <= 0
        {
            answer_popup_message.set(view!{ cx, "Failed..."});
            answer_is_open.set(true);
            is_correct.set(false);
        }
    };

    let button_disabled = move ||
    {
        quiz.get().quiz.len() <= 0
    };

    //  Initializes quiz.
    spawn_local_scoped(cx, async move
    {
        let api_key = app_state.api_key.get();

        if *app_state.save_result.get()
        {
            if let Some(user) = app_state.user.get().as_ref()
            {
                user_result.set
                (
                    Some(create_user_result(&api_key, &user).await)
                );
            }
        }
        update_quiz().await;
    });

    view!
    {
        cx,

        div(class="page_quiz flex column spacer")
        {
            //  Quiz
            div(class="ui_panel shadow radius padding_lg margin_vertical_lg")
            {
                div { "Q. " (quiz_cnt.get()) }
                (
                    {
                        let q = quiz.get();
                        if q.quiz.len() > 0
                        {
                            view!
                            {
                                cx,
                                div
                                (
                                    class="quiz_content margin_top",
                                    dangerously_set_inner_html=&q.quiz,
                                )
                            }
                        }
                        else
                        {
                            view!{ cx, Loading }
                        }
                    }
                )
            }

            div(class="spacer")

            //  Answer
            div(class="ui_panel padding_md margin_bottom_lg")
            {
                label(class="flex column align_start")
                {
                    span(class="ui_label margin_bottom")
                    {
                        "Answer (Remain: " (remain.get()) ")"
                    }
                    input
                    (
                        type="text",
                        class="ui_text full_horizon",
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
                            p(class="margin_top hint")
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

                //  Operation
                div(class="margin_top flex row justify_between")
                {
                    button
                    (
                        class="ui_button error",
                        on:click=move |_| { quit_is_open.set(true); }
                    ) { "Quit" }

                    button
                    (
                        class="ui_button warn",
                        on:click=move |_| { giveup_is_open.set(true); },
                        disabled=button_disabled(),
                    ) { "Give up" }

                    button
                    (
                        class="ui_button warn",
                        on:click=move |_|
                        {
                            spawn_local_scoped(cx, async move
                            {
                                update_quiz().await;
                            });
                        },
                        disabled=button_disabled(),
                    ) { "Skip" }

                    button
                    (
                        class="ui_button info",
                        on:click=move |_| { update_hint(); },
                        disabled=button_disabled(),
                    ) { "Hint" }

                    button
                    (
                        class="ui_button success",
                        on:click=move |_| { try_answer(); },
                        disabled=button_disabled(),
                    ) { "Answer" }
                }
            }

            //  Popups
            QuitPopup(is_open=quit_is_open)
            GiveupPopup
            (
                is_open=giveup_is_open,
                callback=Box::new(move ||
                {
                    giveup_is_open.set(false);
                    answer_popup_message.set(view!{ cx, "Failed..."});
                    answer_is_open.set(true);
                    is_correct.set(false);
                }),
            )
            AnswerPopup
            (
                is_open=answer_is_open,
                quiz=quiz,
                message=answer_popup_message,
                callback=Box::new(move ||
                {
                    answer_is_open.set(false);
                    spawn_local_scoped(cx, async move
                    {
                        terminate_quiz();
                        update_quiz().await;
                    });
                }),
            )
        }
    }
}

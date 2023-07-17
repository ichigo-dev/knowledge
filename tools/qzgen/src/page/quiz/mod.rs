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

use crate::MAX_TRY_CNT;
use crate::functions::generate_quiz;


//------------------------------------------------------------------------------
//  Quiz page component.
//------------------------------------------------------------------------------
#[component]
pub fn Quiz<G: Html>( cx: Scope ) -> View<G>
{
    //  Signals.
    let quiz = create_signal(cx, String::new());
    let answer = create_signal(cx, String::new());
    let answer_path = create_signal(cx, String::new());
    let hint_len = create_signal(cx, 0);
    let hint = create_signal(cx, String::new());

    //  User answer.
    let input_answer = create_signal(cx, String::new());
    let remain = create_signal(cx, MAX_TRY_CNT);

    //  Popup states.
    let quit_is_open = create_signal(cx, false);
    let giveup_is_open = create_signal(cx, false);
    let answer_is_open = create_signal(cx, false);
    let answer_popup_message = create_signal(cx, view!{ cx, });

    //  Generates quiz.
    let update_quiz = || async
    {
        //  Resets signals.
        input_answer.set("".to_string());
        hint_len.set(0);
        hint.set("".to_string());
        remain.set(MAX_TRY_CNT);

        //  Generates next quiz.
        quiz.set("Generating quiz...".to_string());
        let (new_quiz, new_answer, new_answer_path) = generate_quiz().await;
        quiz.set(new_quiz);
        answer.set(new_answer);
        answer_path.set(new_answer_path);
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

        if len >= answer.get().chars().count()
        {
            new_hint.push_str(" (No more hints...)");
        }
        else
        {
            hint_len.set(len);
        }
        hint.set(new_hint);
    };

    //  User answer.
    let try_answer = move ||
    {
        remain.set(*remain.get() - 1);
        if input_answer.get().to_lowercase().trim()
            == answer.get().to_lowercase().trim()
        {
            answer_popup_message.set(view!{ cx, "Great!" });
            answer_is_open.set(true);
        }
        else if *remain.get() <= 0
        {
            answer_popup_message.set(view!{ cx, "Failed..."});
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

        div(class="spacer")

        //  Operation
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
            }),
        )
        AnswerPopup
        (
            is_open=answer_is_open,
            answer=answer,
            answer_path=answer_path,
            message=answer_popup_message,
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

/*

    Answer popup component.

*/

use sycamore::prelude::*;

use crate::component::Popup;
use crate::Quiz;

#[component(inline_props)]
pub fn AnswerPopup<'cx, G: Html>
(
    cx: Scope<'cx>,
    is_open: &'cx Signal<bool>,
    quiz: &'cx Signal<Quiz>,
    message: &'cx Signal<View<G>>,
    callback: Box<dyn Fn() + 'cx>,
) -> View<G>
{
    view!
    {
        cx,
        Popup
        (
            title="Answer",
            child=view!
            {
                cx,
                p(class="margin_bottom")
                {
                    (*message.get())
                }
                p(class="margin_bottom")
                {
                    "Answer: "
                    a
                    (
                        href=&quiz.get().answer_path,
                        target="_blank",
                        rel="noopener noreferrer",
                    )
                    {
                        (
                            {
                                let q = quiz.get();
                                view!
                                {
                                    cx,
                                    (q.term.term)
                                }
                            }
                        )
                    }
                }
                div(class="flex justify_center")
                {
                    button
                    (
                        class="ui_button primary",
                        on:click=move |_| callback()
                    ) { "Next" }
                }
            },
            is_open=is_open,
            close_icon=false,
        )
    }
}

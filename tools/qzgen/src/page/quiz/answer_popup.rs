/*

    Answer popup component.

*/

use sycamore::prelude::*;

use crate::component::Popup;

#[component(inline_props)]
pub fn AnswerPopup<'cx, G: Html>
(
    cx: Scope<'cx>,
    is_open: &'cx Signal<bool>,
    answer: &'cx Signal<String>,
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
                    "Answer: "
                    (answer.get())
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

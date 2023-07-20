/*

    Answer popup component.

*/

use sycamore::prelude::*;

use crate::component::Popup;
use crate::Term;

#[component(inline_props)]
pub fn AnswerPopup<'cx, G: Html>
(
    cx: Scope<'cx>,
    is_open: &'cx Signal<bool>,
    term: &'cx Signal<Option<Term>>,
    answer_path: &'cx Signal<String>,
    message: &'cx Signal<View<G>>,
    callback: Box<dyn Fn() + 'cx>,
) -> View<G>
{
    let term = term.get();
    let answer = match term.as_ref()
    {
        Some(t) => t.term.clone(),
        None => "".to_string(),
    };

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
                    a(href=answer_path.get(), target="_blank")
                    {
                        (answer)
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

/*

    Qiut popup component.

*/

use sycamore::prelude::*;

use crate::component::Popup;

#[component(inline_props)]
pub fn QuitPopup<'cx, G: Html>
(
    cx: Scope<'cx>,
    is_open: &'cx Signal<bool>,
) -> View<G>
{
    view!
    {
        cx,
        Popup
        (
            title="Quit",
            child=view!
            {
                cx,
                p(class="margin_bottom")
                {
                    "Are you sure you want to quit?"
                }
                div(class="flex justify_between")
                {
                    button
                    (
                        class="ui_button error",
                        on:click=move |_|
                        {
                            is_open.set(false);
                        },
                    ) { "Cancel" }
                    a(href="/", class="ui_button primary") { "Done" }
                }
            },
            is_open=is_open,
            close_icon=true,
        )
    }
}

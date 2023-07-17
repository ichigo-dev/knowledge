/*

    Giveup popup component.

*/

use sycamore::prelude::*;

use crate::component::Popup;

#[component(inline_props)]
pub fn GiveupPopup<'cx, G: Html>
(
    cx: Scope<'cx>,
    is_open: &'cx Signal<bool>,
    callback: Box<dyn Fn() + 'cx>,
) -> View<G>
{
    view!
    {
        cx,
        Popup
        (
            title="Give up",
            child=view!
            {
                cx,
                p(class="margin_bottom")
                {
                    "Are you sure you want to give up?"
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
                    button
                    (
                        class="ui_button primary",
                        on:click=move |_| callback()
                    ) { "Done" }
                }
            },
            is_open=is_open,
            close_icon=true,
        )
    }
}

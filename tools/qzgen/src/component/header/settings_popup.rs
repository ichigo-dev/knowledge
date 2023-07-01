use sycamore::prelude::*;
use crate::component::Popup;

#[component(inline_props)]
pub fn SettingsPopup<'cx, G: Html>
(
    cx: Scope<'cx>,
    is_open: &'cx Signal<bool>
) -> View<G>
{
    view!
    {
        cx,
        Popup
        (
            title="Settings",
            child=view!
            {
                cx,
                div { "Hello" }
            },
            is_open=is_open,
        )
    }
}

/*

    Settings popup component.

*/

use super::HeaderState;
use super::theme_select::ThemeSelect;
use crate::component::Popup;

use sycamore::prelude::*;

#[component]
pub fn SettingsPopup<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    let header_state = use_context::<HeaderState>(cx);
    let is_open = create_signal(cx, *header_state.settings_popup_is_open.get());
    create_effect(cx, ||
    {
        header_state.settings_popup_is_open.set(*is_open.get());
    });

    view!
    {
        cx,
        Popup
        (
            title="Settings",
            child=view!
            {
                cx,
                div(class="flex column gap_md")
                {
                    ThemeSelect
                }
            },
            is_open=is_open,
        )
    }
}

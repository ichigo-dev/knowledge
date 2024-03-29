/*

    Settings popup component.

*/

use super::save_result_toggle::SaveResultToggle;
use super::theme_select::ThemeSelect;
use super::api_key_input::ApiKeyInput;
use crate::component::Popup;

use sycamore::prelude::*;

#[component(inline_props)]
pub fn SettingsPopup<'cx, G: Html>
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
            title="Settings",
            child=view!
            {
                cx,
                div(class="settings_popup flex column align_start gap_md")
                {
                    SaveResultToggle
                    ThemeSelect
                    ApiKeyInput
                }
            },
            is_open=is_open,
            close_icon=true,
        )
    }
}

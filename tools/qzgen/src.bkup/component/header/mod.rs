mod settings_icon;
mod settings_popup;
mod theme_select;
mod language_select;

use settings_icon::SettingsIcon;

use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        header(class="header")
        {
            SettingsIcon
        }
    }
}

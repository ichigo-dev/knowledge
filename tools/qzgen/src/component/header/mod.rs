mod settings_icon;
mod settings_popup;

use settings_icon::SettingsIcon;
use crate::app::AppState;
use crate::theme::Theme;
use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);

    //  Toggle dark mode.
    let checked = create_signal(cx, false);
    let toggle_theme = |_|
    {
        if *checked.get() == true
        {
            app_state.set_theme(Theme::Light);
        }
        else
        {
            app_state.set_theme(Theme::Dark);
        }
    };

    view!
    {
        cx,
        header(class="header")
        {
            label(class="margin_right_sm")
            {
                span(class="ui_label margin_right_sm") { "Dark Mode" }
                input
                (
                    class="ui_toggle",
                    type="checkbox",
                    on:change=toggle_theme,
                    bind:checked=checked,
                )
            }
            SettingsIcon
        }
    }
}

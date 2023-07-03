use crate::theme::Theme;
use crate::app::AppState;

use sycamore::prelude::*;

#[component]
pub fn ThemeSelect<G: Html>( cx: Scope ) -> View<G>
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
        label
        {
            span(class="ui_label margin_right_sm") { "Dark Mode: " }
            input
            (
                class="ui_toggle",
                type="checkbox",
                on:change=toggle_theme,
                bind:checked=checked,
            )
        }
    }
}

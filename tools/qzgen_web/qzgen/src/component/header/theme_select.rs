/*

    Theme select toggle component.

*/

use sycamore::prelude::*;

use crate::app::AppState;
use crate::theme::Theme;

#[component]
pub fn ThemeSelect<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);

    //  Toggle dark mode.
    let checked = create_signal(cx, app_state.theme.get() == Theme::Dark.into());
    let toggle_theme = |_|
    {
        if *checked.get() == true
        {
            app_state.theme.set(Theme::Light);
        }
        else
        {
            app_state.theme.set(Theme::Dark);
        }

        //  Saves to local storage.
        let local_storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .expect("local storage should be available");
        local_storage
            .set_item("theme", &(*app_state.theme.get()).to_string())
            .unwrap();
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

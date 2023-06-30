use crate::app::AppState;
use crate::theme::Theme;
use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);

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
            h1(class="page_title") { "Quiz Generator" }
            label(class="theme_toggle")
            {
                input
                (
                    class="ui_toggle",
                    type="checkbox",
                    on:change=toggle_theme,
                    bind:checked=checked,
                )
                span(class="ui_label") { "Dark Mode" }
            }
        }
    }
}

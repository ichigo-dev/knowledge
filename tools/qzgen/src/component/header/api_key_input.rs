/*

    API Key input component.

*/

use sycamore::prelude::*;

use crate::app::AppState;

#[component]
pub fn ApiKeyInput<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);
    let api_key = create_signal(cx, app_state.api_key.get().to_string());

    create_effect(cx, ||
    {
        app_state.api_key.set(api_key.get().to_string());

        let local_storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .expect("local storage should be available");
        local_storage.set_item("api_key", &api_key.get()).unwrap();
    });

    view!
    {
        cx,
        label
        {
            span(class="ui_label margin_right_sm") { "API Key: " }
            input
            (
                class="ui_text",
                type="text",
                bind:value=api_key,
            )
        }
    }
}

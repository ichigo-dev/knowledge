/*

    Toggle save result component.

*/

use sycamore::prelude::*;

use crate::app::AppState;

#[component]
pub fn SaveResultToggle<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);

    let checked = create_signal(cx, *app_state.save_result.get());
    let toggle_save_result = |_|
    {
        app_state.save_result.set(!(*checked.get()));

        //  Saves to local storage.
        let local_storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .expect("local storage should be available");
        local_storage
            .set_item("save_result", &(*app_state.save_result.get()).to_string())
            .unwrap();
    };

    view!
    {
        cx,
        label
        {
            span(class="ui_label margin_right_sm") { "Save result: " }
            input
            (
                class="ui_toggle",
                type="checkbox",
                on:change=toggle_save_result,
                bind:checked=checked,
            )
        }
    }
}

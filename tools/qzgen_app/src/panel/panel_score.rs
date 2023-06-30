use sycamore::prelude::*;

#[component]
pub fn PanelScore<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="ui_panel panel_score")
        {
            p { "panel score" }
        }
    }
}

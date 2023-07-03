use sycamore::prelude::*;

#[component]
pub fn ScorePanel<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="ui_panel margin_vertical padding")
        {
            p { "panel score" }
        }
    }
}

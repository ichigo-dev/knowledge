use sycamore::prelude::*;

#[component]
pub fn PanelAnswer<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="ui_panel panel_answer")
        {
            p { "panel answer" }
        }
    }
}

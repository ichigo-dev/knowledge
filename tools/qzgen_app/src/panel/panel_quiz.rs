use sycamore::prelude::*;

#[component]
pub fn PanelQuiz<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="ui_panel panel_quiz")
        {
            p { "panel quiz " }
        }
    }
}

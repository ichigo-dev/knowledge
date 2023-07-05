use sycamore::prelude::*;

#[component]
pub fn AnswerPanel<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="ui_panel margin_vertical padding")
        {
            button(class="ui_button primary") { "Next" }
        }
    }
}

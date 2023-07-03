use sycamore::prelude::*;
use sycamore::futures::create_resource;

#[component]
pub fn QuizPanel<G: Html>( cx: Scope ) -> View<G>
{
    let quiz = create_signal(cx, "".to_string());
    create_resource(cx, async move
    {
    });

    view!
    {
        cx,
        div
        (
            dangerously_set_inner_html=&quiz.get(),
            class="ui_panel margin_vertical padding",
        )
    }
}

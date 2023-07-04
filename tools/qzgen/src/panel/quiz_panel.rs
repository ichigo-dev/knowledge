use crate::app::AppState;
use crate::term::Term;

use sycamore::prelude::*;
use sycamore::futures::create_resource;

#[component]
pub async fn QuizPanel<G: Html>( cx: Scope<'_> ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);
    let terms = use_context::<RcSignal<Vec<Term>>>(cx);

    let lang_code = app_state.language().code();
    let term = terms.get();
    let term = term.get(0).unwrap();
    let quiz = term.generate_quiz(&lang_code).await;

    view!
    {
        cx,
        div
        (
            dangerously_set_inner_html=&quiz,
            class="ui_panel margin_vertical padding",
        )
    }
}

use crate::api::GitHubApi;
use crate::app::AppState;

use sycamore::prelude::*;
use sycamore::futures::create_resource;

#[component]
pub fn QuizPanel<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);

    let quiz = create_signal(cx, "".to_string());
    create_resource(cx, async move
    {
        log::info!("test");
        let quiz_no = app_state.quiz_no();
        let terms = app_state.terms();
        let term = &terms[*quiz_no];
        let path = term.path();
        let contents = GitHubApi::new().get_content(path).await;
        quiz.set(contents);
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

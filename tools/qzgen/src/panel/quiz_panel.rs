use crate::api::GitHubApi;

use sycamore::prelude::*;
use sycamore::futures::create_resource;

#[component]
pub fn QuizPanel<G: Html>( cx: Scope ) -> View<G>
{
    let quiz = create_signal(cx, "".to_string());
    create_resource(cx, async move
    {
        /*
        let api = GitHubApi::new();
        match api.get_content("README.md").await
        {
            Some(s) => quiz.set(s),
            None => quiz.set("".to_string()),
        };
        */
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

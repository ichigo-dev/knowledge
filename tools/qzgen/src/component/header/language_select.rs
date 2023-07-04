use crate::language::Language;
use crate::app::AppState;
use crate::term::Term;
use crate::api::GitHubApi;

use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;

#[component]
pub fn LanguageSelect<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);
    let terms = use_context::<RcSignal<Vec<Term>>>(cx);

    let all_languages = Language::get_all_languages();
    let select_item = create_signal(cx, all_languages);

    let language = create_signal(cx, app_state.language().code().to_string());

    create_effect
    (
        cx,
        move ||
        {
            let new_language = Language::from(language.get().as_str());
            app_state.set_language(new_language);

            spawn_local_scoped(cx, async
            {
                let api = GitHubApi::new();
                terms.set(api.get_terms(&app_state.language().code()).await);
            });
        }
    );

    view!
    {
        cx,
        label
        {
            span(class="ui_label margin_right_sm") { "Quiz Language: " }
            select
            (
                class="ui_select",
                bind:value=language,
            )
            {
                Indexed
                (
                    iterable=select_item,
                    view=move |cx, item: Language|
                    {
                        let code = item.code();
                        let code_value = item.code();
                        view!
                        {
                            cx,
                            option
                            (
                                value=code_value,
                                selected=language.get().as_str() == code,
                            )
                            {
                                (item.label())
                            }
                        }
                    }
                )
            }
        }
    }
}

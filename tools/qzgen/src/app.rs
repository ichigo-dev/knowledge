use std::default::Default;
use std::rc::Rc;

use crate::theme::Theme;
use crate::language::Language;
use crate::term::Term;
use crate::component::Header;
use crate::panel::{ QuizPanel, AnswerPanel, ScorePanel };
use crate::api::GitHubApi;

use sycamore::prelude::*;
use sycamore::suspense::Suspense;

pub struct AppState
{
    theme: RcSignal<Theme>,
    language: RcSignal<Language>,
}

impl AppState
{
    pub fn theme( &self ) -> Rc<Theme>
    {
        self.theme.get()
    }

    pub fn set_theme( &self, theme: Theme )
    {
        self.theme.set(theme);
    }

    pub fn language( &self ) -> Rc<Language>
    {
        self.language.get()
    }

    pub fn set_language( &self, language: Language )
    {
        self.language.set(language);
    }
}

impl Default for AppState
{
    fn default() -> Self
    {
        AppState
        {
            theme: create_rc_signal(Theme::default()),
            language: create_rc_signal(Language::default()),
        }
    }
}

#[component]
pub fn App<G: Html>( cx: Scope ) -> View<G>
{
    let state = AppState::default();
    let app_state = provide_context(cx, state);
    let class = ||
    {
        "wrapper ".to_string() + &app_state.theme().get_class_name()
    };

    view!
    {
        cx,
        div(class=class())
        {
            Suspense(fallback=view! { cx, p { "Loading..." } })
            {
                Provider
            }
        }
    }
}

#[component]
async fn Provider<G: Html>( cx: Scope<'_> ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);
    let terms = provide_context(cx, create_rc_signal(Vec::new()));

    let api = GitHubApi::new();
    terms.set(api.get_terms(&app_state.language().code()).await);

    view!
    {
        cx,
        div(class="inner")
        {
            Header
            h1(class="page_title") { "Quiz Generator" }
            Suspense(fallback=view! { cx, p { "Loading..." } })
            {
                QuizPanel
            }
            AnswerPanel
            ScorePanel
        }
    }
}

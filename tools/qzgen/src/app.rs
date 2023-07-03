use std::default::Default;
use std::rc::Rc;

use crate::theme::Theme;
use crate::language::Language;
use crate::term::Term;
use crate::component::Header;
use crate::panel::{ QuizPanel, AnswerPanel, ScorePanel };
use crate::api::GitHubApi;

use sycamore::prelude::*;
use sycamore::futures::create_resource;

pub struct AppState
{
    theme: RcSignal<Theme>,
    language: RcSignal<Language>,
    terms: RcSignal<Vec<Term>>,
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

    pub fn terms( &self ) -> Rc<Vec<Term>>
    {
        self.terms.get()
    }

    pub fn set_terms( &self, terms: Vec<Term> )
    {
        self.terms.set(terms);
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
            terms: create_rc_signal(Vec::new()),
        }
    }
}

#[component]
pub fn App<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = provide_context(cx, AppState::default());
    let class = ||
    {
        "wrapper ".to_string() + &app_state.theme().get_class_name()
    };

    create_resource(cx, async move
    {
        let api = GitHubApi::new();
        let checksheet_path = format!
        (
            "note/{}/checksheet.md",
            app_state.language().code(),
        );

        match api.get_content(&checksheet_path).await
        {
            Some(s) =>
            {
            },
            None => {},
        };
    });

    view!
    {
        cx,
        div(class=class())
        {
            div(class="inner")
            {
                Header
                h1(class="page_title") { "Quiz Generator" }
                QuizPanel
                AnswerPanel
                ScorePanel
            }
        }
    }
}

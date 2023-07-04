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
    quiz_no: RcSignal<usize>,
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

    pub fn quiz_no( &self ) -> Rc<usize>
    {
        self.quiz_no.get()
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
            quiz_no: create_rc_signal(0),
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
        let terms = api.get_terms(&app_state.language().code()).await;
        app_state.set_terms(terms);
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

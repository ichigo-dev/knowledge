use std::default::Default;
use std::rc::Rc;
use std::fs;

use crate::theme::Theme;
use crate::language::Language;
use crate::component::Header;
use crate::panel::{ QuizPanel, AnswerPanel, ScorePanel };

use sycamore::prelude::*;
use sycamore::futures::create_resource;

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
    let app_state = provide_context(cx, AppState::default());
    let class = ||
    {
        "wrapper ".to_string() + &app_state.theme().get_class_name()
    };

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

use std::default::Default;
use std::rc::Rc;
use std::path::PathBuf;

use crate::theme::Theme;
use crate::component::Header;
use crate::panel::{ PanelQuiz, PanelAnswer, PanelScore };
use sycamore::prelude::*;

pub struct AppState
{
    theme: RcSignal<Theme>,
    checksheet_path: RcSignal<PathBuf>,
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

    pub fn checksheet_path( &self ) -> Rc<PathBuf>
    {
        self.checksheet_path.get()
    }

    pub fn set_checksheet_path( &self, checksheet_path: PathBuf )
    {
        self.checksheet_path.set(checksheet_path);
    }
}

impl Default for AppState
{
    fn default() -> Self
    {
        let checksheet_path = PathBuf::from("./note/ja/checksheet.md");
        AppState
        {
            theme: create_rc_signal(Theme::default()),
            checksheet_path: create_rc_signal(checksheet_path),
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
                PanelQuiz
                PanelAnswer
                PanelScore
            }
        }
    }
}

use std::default::Default;
use std::rc::Rc;
use crate::theme::Theme;
use crate::common::Header;
use crate::panel::{ PanelQuiz, PanelAnswer, PanelScore };
use sycamore::prelude::*;

pub struct AppState
{
    pub theme: RcSignal<Theme>,
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
}

impl Default for AppState
{
    fn default() -> Self
    {
        AppState
        {
            theme: create_rc_signal(Theme::default()),
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
                PanelQuiz
                PanelAnswer
                PanelScore
            }
        }
    }
}

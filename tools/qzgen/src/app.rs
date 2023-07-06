/*

    Application base.

*/

use std::default::Default;

use sycamore::prelude::*;

use crate::theme::Theme;
use crate::component::Header;


//------------------------------------------------------------------------------
//  Application State.
//------------------------------------------------------------------------------
pub struct AppState
{
    pub theme: RcSignal<Theme>,
}

impl Default for AppState
{
    //--------------------------------------------------------------------------
    //  Gets the default application state.
    //--------------------------------------------------------------------------
    fn default() -> Self
    {
        Self
        {
            theme: create_rc_signal(Theme::default()),
        }
    }
}


//------------------------------------------------------------------------------
//  Application base component.
//------------------------------------------------------------------------------
#[component]
pub fn App<G: Html>( cx: Scope ) -> View<G>
{
    let app_state = provide_context(cx, AppState::default());
    let class = ||
    {
        "wrapper ".to_string() + &app_state.theme.get().class_name()
    };

    view!
    {
        cx,
        div(class=class())
        {
            Header
            "Hello, World"
        }
    }
}

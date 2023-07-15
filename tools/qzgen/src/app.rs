/*

    Application base.

*/

use std::default::Default;

use sycamore::prelude::*;
use sycamore_router::{ Route, Router, HistoryIntegration };

use crate::theme::Theme;
use crate::component::Header;
use crate::page::{ Home, Quiz, NotFound };


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
//  Application router.
//------------------------------------------------------------------------------
#[derive(Route)]
enum AppRoute
{
    #[to("/")]
    Home,

    #[to("/quiz")]
    Quiz,

    #[not_found]
    NotFound,
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
        Router
        (
            integration=HistoryIntegration::new(),
            view=move |cx, route: &ReadSignal<AppRoute>|
            {
                view!
                {
                    cx,
                    div(class=class())
                    {
                        div(class="inner")
                        {
                            Header
                            div(class="flex column spacer")
                            {
                                (match route.get().as_ref()
                                {
                                    AppRoute::Home => view! { cx, Home },
                                    AppRoute::Quiz => view! { cx, Quiz },
                                    AppRoute::NotFound => view! { cx, NotFound },
                                })
                            }
                        }
                    }
                }
            }
        )
    }
}

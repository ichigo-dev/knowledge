/*

    Application base.

*/

use std::default::Default;

use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use sycamore_router::{ Route, Router, HistoryIntegration };

use crate::data::User;
use crate::theme::Theme;
use crate::component::Header;
use crate::page::{ Home, Quiz, NotFound };
use crate::functions::get_or_create_user;


//------------------------------------------------------------------------------
//  Application State.
//------------------------------------------------------------------------------
pub struct AppState
{
    pub theme: RcSignal<Theme>,
    pub user: RcSignal<Option<User>>,
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
            user: create_rc_signal(None),
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

    spawn_local_scoped(cx, async move
    {
        let user = get_or_create_user().await;
        app_state.user.set(Some(user));
    });

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

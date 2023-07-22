/*

    Application base.

*/

use std::default::Default;
use std::str::FromStr;

use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::{ Route, Router, HistoryIntegration };

use crate::data::User;
use crate::theme::Theme;
use crate::component::{ Header, Loading };
use crate::page::{ Home, Quiz, Record, NotFound };
use crate::functions::get_or_create_user;


//------------------------------------------------------------------------------
//  Application State.
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct AppState
{
    pub theme: RcSignal<Theme>,
    pub user: RcSignal<Option<User>>,
    pub api_key: RcSignal<String>,
    pub save_result: RcSignal<bool>,
}

impl Default for AppState
{
    //--------------------------------------------------------------------------
    //  Gets the default application state.
    //--------------------------------------------------------------------------
    fn default() -> Self
    {
        //  Gets state from local storage.
        let local_storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .expect("local storage should be available");
        let mut theme = Theme::default();
        let mut save_result = true;
        let mut api_key = String::new();
        if let Ok(Some(t)) = local_storage.get_item("theme")
        {
            theme = Theme::from_str(&t).unwrap();
        }
        if let Ok(Some(s)) = local_storage.get_item("save_result")
        {
            save_result = s == "true";
        }
        if let Ok(Some(a)) = local_storage.get_item("api_key")
        {
            api_key = a;
        }

        Self
        {
            theme: create_rc_signal(theme),
            user: create_rc_signal(None),
            api_key: create_rc_signal(api_key),
            save_result: create_rc_signal(save_result),
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

    #[to("/record")]
    Record,

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
                            Suspense(fallback=view! { cx, Loading })
                            {
                                MainContent(route=route)
                            }
                        }
                    }
                }
            }
        )
    }
}


//------------------------------------------------------------------------------
//  Application main content.
//------------------------------------------------------------------------------
#[component(inline_props)]
async fn MainContent<'cx, G: Html>
(
    cx: Scope<'cx>,
    route: &'cx ReadSignal<AppRoute>,
) -> View<G>
{
    //  Initializes the user.
    let app_state = use_context::<AppState>(cx);
    let user = get_or_create_user(&app_state.api_key.get()).await;
    app_state.user.set(Some(user));

    view!
    {
        cx,
        div(class="flex column spacer")
        {
            (match route.get().as_ref()
            {
                AppRoute::Home => view! { cx, Home },
                AppRoute::Quiz => view! { cx, Quiz },
                AppRoute::Record => view! { cx, Record },
                AppRoute::NotFound => view!
                {
                    cx,
                    NotFound
                },
            })
        }
    }
}

/*

    Application header.

*/

mod settings_icon;
mod settings_popup;
mod theme_select;

use std::default::Default;

use sycamore::prelude::*;

use settings_icon::SettingsIcon;
use settings_popup::SettingsPopup;


//------------------------------------------------------------------------------
//  Header state.
//------------------------------------------------------------------------------
pub struct HeaderState
{
    pub settings_popup_is_open: RcSignal<bool>,
}

impl Default for HeaderState
{
    //--------------------------------------------------------------------------
    //  Gets the default hedaer state.
    //--------------------------------------------------------------------------
    fn default() -> Self
    {
        Self
        {
            settings_popup_is_open: create_rc_signal(false),
        }
    }
}


//------------------------------------------------------------------------------
//  Header component.
//------------------------------------------------------------------------------
#[component]
pub fn Header<G: Html>( cx: Scope ) -> View<G>
{
    let header_state = provide_context(cx, HeaderState::default());

    view!
    {
        cx,
        header(class="header")
        {
            SettingsIcon
        }

        //  Settings popup.
        (
            if *header_state.settings_popup_is_open.get()
            {
                view! { cx, SettingsPopup }
            }
            else
            {
                view! { cx, }
            }
        )
    }
}

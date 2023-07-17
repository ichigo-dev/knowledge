/*

    Application header.

*/

mod settings_popup;
mod theme_select;

use sycamore::prelude::*;

use crate::component::HomeIcon;
use crate::component::SettingsIcon;
use settings_popup::SettingsPopup;


//------------------------------------------------------------------------------
//  Header component.
//------------------------------------------------------------------------------
#[component]
pub fn Header<G: Html>( cx: Scope ) -> View<G>
{
    let is_open = create_signal(cx, false);

    view!
    {
        cx,
        header(class="header margin_bottom")
        {
            HomeIcon
            SettingsIcon(callback=Some(Box::new(move ||
            {
                is_open.set(true);
            })))
        }
        SettingsPopup(is_open=is_open)
    }
}

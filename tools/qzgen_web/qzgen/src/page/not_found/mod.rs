/*

    404 page.

*/

use sycamore::prelude::*;


//------------------------------------------------------------------------------
//  404 page component.
//------------------------------------------------------------------------------
#[component]
pub fn NotFound<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="ui_panel padding_lg flex column align_center")
        {
            "404 Not Found"
        }
    }
}

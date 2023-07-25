/*

    Loading indicator.

*/

use sycamore::prelude::*;

#[component]
pub fn Loading<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="padding_lg flex column align_center full_vertical full_horizon")
        {
            div(class="ui_loader")
            {
                div {}
                div {}
                div {}
                div {}
                div {}
                div {}
                div {}
                div {}
            }
        }
    }
}

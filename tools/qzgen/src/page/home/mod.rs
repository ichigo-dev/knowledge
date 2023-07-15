/*

    Home page.

*/

use sycamore::prelude::*;


//------------------------------------------------------------------------------
//  Home page component.
//------------------------------------------------------------------------------
#[component]
pub fn Home<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="padding_lg flex column align_center")
        {
            a(href="/quiz", class="ui_button primary") { "Start Quiz" }
        }
    }
}

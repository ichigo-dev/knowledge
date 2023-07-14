/*

    Quiz panel.

*/

use sycamore::prelude::*;


//------------------------------------------------------------------------------
//  Quiz panel component.
//------------------------------------------------------------------------------
#[component]
pub fn QuizPanel<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="ui_panel")
        {
        }
    }
}

/*

    Home page.

*/

use sycamore::prelude::*;

use crate::component::{ QuizIcon, RecordIcon };


//------------------------------------------------------------------------------
//  Home page component.
//------------------------------------------------------------------------------
#[component]
pub fn Home<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="home_page padding_lg flex column align_center gap_md")
        {
            h2(class="app_name")
            {
                "QZgen"
            }
            div(class="flex justify_between gap_md full_horizon")
            {
                a(href="/quiz", class="home_menu_button")
                {
                    QuizIcon
                    "Challenge the Quiz"
                }
                a(href="/record", class="home_menu_button")
                {
                    RecordIcon
                    "View Past Records"
                }
            }
        }
    }
}

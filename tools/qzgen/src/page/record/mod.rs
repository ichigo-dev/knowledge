/*

    Record page.

*/

mod record_list;

use sycamore::prelude::*;
use sycamore::suspense::Suspense;

use record_list::RecordList;
use crate::component::Loading;


//------------------------------------------------------------------------------
//  Record page component.
//------------------------------------------------------------------------------
#[component]
pub fn Record<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div(class="padding_lg flex column align_center")
        {
            Suspense(fallback=view! { cx, Loading })
            {
                RecordList
            }
        }
    }
}

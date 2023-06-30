use sycamore::prelude::*;
use crate::component::Popup;

#[component]
pub fn SettingsPopup<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        Popup(child=view!
        {
            cx,
            div { "Hello" }
        })
    }
}

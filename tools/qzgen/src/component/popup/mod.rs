/*

    Common popup component.
    This component is used to display a popup with a close button.

    # Example

    ```rs
    use sycamore::prelude::*;
    use crate::component::Popup;

    #[component(inline_props)]
    pub fn SomethingComponent<G: Html>( cx: Scope ) -> View<G>
    {
        let is_open = create_signal(cx, true);

        view!
        {
            cx,
            button(on:click=|_| is_open.set(true))
            {
                "Open popup"
            }

            (
                if *is_open.get()
                {
                    view!
                    {
                        cx,
                        Popup
                        (
                            child=view!{ cx, "Hello, world!" },
                            is_open=is_open,
                        )
                    }
                }
                else
                {
                    view! { cx, }
                }
            )
        }
    }
    ```

*/

use sycamore::prelude::*;

#[component(inline_props)]
pub fn Popup<'cx, G: Html>
(
    cx: Scope<'cx>,
    title: &'static str,
    child: View<G>,
    is_open: &'cx Signal<bool>,
) -> View<G>
{
    view!
    {
        cx,
        div(class="ui_popup show")
        {
            div(class="content")
            {
                div(class="header")
                {
                    h4(class="title") { (title) }
                    CloseIcon(is_open=is_open)
                }
                div(class="body")
                {
                    (child)
                }
                div(class="footer")
                {
                }
            }
        }
    }
}

#[component(inline_props)]
fn CloseIcon<'cx, G: Html>
(
    cx: Scope<'cx>,
    is_open: &'cx Signal<bool>,
) -> View<G>
{
    view!
    {
        cx,
        svg
        (
            class="ui_icon clickable",
            xmlns="http://www.w3.org/2000/svg",
            viewBox="0 0 384 512",
            on:click=move |_| is_open.set(false),
        )
        {
            path(d="M342.6 150.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192 210.7 86.6 105.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L146.7 256 41.4 361.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 301.3 297.4 406.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.3 256 342.6 150.6z")
        }
    }
}

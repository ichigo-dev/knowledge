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
                            title="Popup title",
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

use crate::component::CloseIcon;

#[component(inline_props)]
pub fn Popup<'cx, G: Html>
(
    cx: Scope<'cx>,
    title: &'static str,
    child: View<G>,
    is_open: &'cx Signal<bool>,
    close_icon: bool,
) -> View<G>
{
    view!
    {
        cx,
        (
            if *is_open.get()
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

                                (
                                    if close_icon
                                    {
                                        view!
                                        {
                                            cx,
                                            CloseIcon(callback=Some(Box::new(||
                                            {
                                                is_open.set(false);
                                            })))
                                        }
                                    }
                                    else
                                    {
                                        view! { cx, }
                                    }
                                )
                            }
                            div(class="body")
                            {
                                (child)
                            }
                        }
                    }
                }
            }
            else
            {
                view! { cx, }
            }
        )
    }
}


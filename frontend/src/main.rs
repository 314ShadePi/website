mod components;
mod core;

use dioxus::prelude::*;

use crate::components::pages::core::page;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
            Router {
                page::page {
                    to: "/",
                    name: "Home",
                    should_be_on_navbar: false,
                    content: cx.render(rsx! {
                        div {
                            p { "Home" }
                            Link { to: "/about", "About" }
                        }
                    })
                }
                page::page {
                    to: "/about",
                    name: "Home",
                    should_be_on_navbar: true,
                    content: cx.render(rsx! {
                        div {
                            p { "About" }
                            Link { to: "/", "Home" }
                        }
                    })
                }
            }
    })
}

mod components;
mod core;

use dioxus::prelude::*;

use crate::components::pages::core::page;
use crate::core::s_page;

fn main() {
    dioxus::web::launch(app);
}

static PAGES: state::Storage<Vec<s_page::Page>> = state::Storage::new();

fn app(cx: Scope) -> Element {
    PAGES.set(vec![s_page::Page { to: "/", name: "Home" }]);
    cx.render(rsx! {
            Router {
                page::page {
                    state: &PAGES,
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
                    state: &PAGES,
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

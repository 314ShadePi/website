mod components;
mod core;

use dioxus::prelude::*;

use crate::components::pages::core::page;
use crate::core::s_page;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let pages = use_state(&cx, || {
        vec![s_page::Page {
            to: "/",
            name: "Home",
        }]
    });

    let page_oncreate = move |page: s_page::Page| {
        if !pages.contains(&page) {
            pages.with_mut(|pages| pages.append(&mut vec![page]));
        }
        gloo::console::log!(format!("{:#?}", pages));
    };

    cx.render(rsx! {
            Router {
                page::page {
                    to: "/",
                    name: "Home",
                    should_be_on_navbar: true,
                    content: cx.render(rsx! {
                        div {
                            p { "Home" }
                            Link { to: "/about", "About" }
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/about",
                    name: "About",
                    should_be_on_navbar: true,
                    content: cx.render(rsx! {
                        div {
                            p { "About" }
                            Link { to: "/", "Home" }
                        }
                    }),
                    oncreate: page_oncreate
                }
            }
    })
}

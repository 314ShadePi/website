mod components;

use dioxus::prelude::*;

use crate::components::pages::core::{footer, page};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let pages = use_state(&cx, || {
        vec![page::Page {
            to: "/",
            name: "Home",
        }]
    });

    let page_oncreate = move |page: page::Page| {
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
                    oncreate: page_oncreate,
                    pages: pages.current()
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
                    pages: pages.current()
                }
            }
            div {
                id: "space-bottom"
            }
            footer::footer {}
    })
}

use std::rc::Rc;

use dioxus::prelude::*;

use super::page::Page;

#[derive(Props)]
pub struct HeaderProps<'a> {
    active_route: &'a str,
    pages: Rc<Vec<Page>>,
}

pub fn header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    cx.render(rsx! {
        header {
            class: "header",
            id: "header-main",
            div {
                class: "container",
                div {
                    class: "row align-items-center justify-content-between",
                    div {
                        class: "logo",
                        span {
                            class: "navbar-logo",
                            Link {
                                to: "/",
                                "314ShadePi"
                            }
                        }
                    }
                    nav {
                        class: "nav",
                        ul {
                            cx.props.pages.iter().map(|page| {
                                let active = if page.to == cx.props.active_route {
                                    "active"
                                } else {
                                    "inactive"
                                };
                                if page.display == true || page.to == cx.props.active_route {
                                    rsx! {
                                        li { class: "display", Link { to: "{page.to}", class: "{active}", "{page.name}" } }
                                    }
                                } else {
                                    let li_class = if page.to == "/escroom" {"display-none secret"} else {"display-none"};
                                    rsx! { 
                                        li { class: "{li_class}", Link { to: "{page.to}", class: "{active}", "{page.name}" } }
                                    }
                                }
                        })
                        }
                    }
                }
            }
            hr {
                id: "header-line"
            }
        }
    })
}

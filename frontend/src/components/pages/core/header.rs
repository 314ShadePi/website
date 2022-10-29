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
                                if page.display == true {
                                    let active = if page.to == cx.props.active_route {
                                        "active"
                                    } else {
                                        "inactive"
                                    };
                                    rsx! {
                                        li { Link { to: "{page.to}", class: "{active}", "{page.name}" } }
                                    }
                                } else {
                                    rsx! { p { id: "nav-no-display"} }
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

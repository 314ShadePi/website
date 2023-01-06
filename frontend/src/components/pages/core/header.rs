use std::rc::Rc;

use dioxus::prelude::*;

use super::page::Page;

#[derive(Props, PartialEq)]
pub struct HeaderProps {
    pages: Rc<Vec<Page>>,
}

pub fn header(cx: Scope<HeaderProps>) -> Element {
    let nav_toggler = include_str!("../../../raw_html/nav_toggler.html");

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
                            class: "navbar-logo big",
                            Link {
                                to: "/",
                                "314ShadePi"
                            }
                        }
                        span {
                            class: "navbar-logo small",
                            Link {
                                to: "/",
                                "314"
                            }
                        }
                    }
                    span {
                        dangerous_inner_html: "{nav_toggler}",
                        nav {
                            class: "nav",
                            ul {
                                cx.props.pages.iter().map(|page| {
                                    if page.display == true {
                                        rsx! {
                                            li { class: "display", Link { to: "{page.to}", "{page.name}" } }
                                        }
                                    } else {
                                        let li_class = if page.to == "/1ryqe-3aakenrScHyF4T6A9LTg7rw4Sk2LmpUlCtrjWASMBKvmtMTkW62up198TtDwPxQr5U5Ew0CfcONSQz2JnAr7cK_5MtZkGGjF3xVFS-RQuOWyxuBDI0y2-YSb6Kc4BQuaWHsW_IOk6RgXl3iqS1jQ_3-W4kcH6EmDn5uY488k3QWoOKs3eg-E20ByHJXiA2VQJqpU_qCrQEKioBaD0bKzFw" {
                                            "display-none secret"
                                        } else {
                                            "display-none"
                                        };
                                        rsx! {
                                            li { class: "{li_class}", Link { to: "{page.to}", "{page.name}" } }
                                        }
                                    }
                                })
                                li { class: "display", Link { to: "https://github.com/314ShadePi", external: true, new_tab: true, "Github" }}
                            }
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

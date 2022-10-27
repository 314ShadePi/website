use std::rc::Rc;

use dioxus::prelude::*;

use crate::core::s_page;

#[derive(Props)]
pub struct HeaderProps<'a> {
    active_route: &'a str,
    pages: Rc<Vec<s_page::Page>>,
}

// Something is not working here
pub fn header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            "Header: active_route = {cx.props.active_route}"
            ul {
                cx.props.pages.iter().map(|page| rsx! {
                    li { Link { to: "{page.to}", "{page.name}" } }
                })
            }
        }
    })
}

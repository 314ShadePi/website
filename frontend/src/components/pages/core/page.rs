use std::rc::Rc;

use crate::core::s_page;

use super::header;
use dioxus::prelude::*;

#[derive(Props)]
pub struct PageProps<'a> {
    to: &'static str,
    name: &'static str,
    should_be_on_navbar: bool,
    content: Element<'a>,
    oncreate: EventHandler<'a, s_page::Page>,
    pages: Rc<Vec<s_page::Page>>,
}
// https://docs.rs/dioxus/latest/dioxus/prelude/struct.EventHandler.html
pub fn page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    if cx.props.should_be_on_navbar == true {
        cx.props.oncreate.call(s_page::Page {
            to: cx.props.to,
            name: cx.props.name,
        });
    }

    cx.render(rsx! {
        Route {
            to: "{cx.props.to}",
            header::header { active_route: cx.props.to, pages: cx.props.pages.clone() }
            &cx.props.content
        }
    })
}

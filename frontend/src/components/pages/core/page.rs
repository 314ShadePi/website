use std::rc::Rc;

use super::header;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Page {
    pub to: &'static str,
    pub name: &'static str,
}

#[derive(Props)]
pub struct PageProps<'a> {
    to: &'static str,
    name: &'static str,
    should_be_on_navbar: bool,
    content: Element<'a>,
    oncreate: EventHandler<'a, Page>,
    pages: Rc<Vec<Page>>,
}

pub fn page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    if cx.props.should_be_on_navbar == true {
        cx.props.oncreate.call(Page {
            to: cx.props.to,
            name: cx.props.name,
        });
    }

    cx.render(rsx! {
        Route {
            to: "{cx.props.to}",
            header::header { active_route: cx.props.to, pages: cx.props.pages.clone() }
            div {
                id: "space-top"
            }
            div {
                class: "main-container",
                &cx.props.content
            }
        }
    })
}

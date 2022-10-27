use super::header;
use crate::core::state;
use dioxus::prelude::*;

#[derive(Props)]
pub struct PageProps<'a> {
    to: &'static str,
    name: &'static str,
    should_be_on_navbar: bool,
    content: Element<'a>,
}

pub fn page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    if cx.props.should_be_on_navbar == true {
        let pages = use_atom_ref(&cx, state::PAGES);
        let mut pages_vec = pages.read().to_vec();
        let mut page = vec![state::Page {
            to: cx.props.to,
            name: cx.props.name,
        }];
        pages_vec.append(&mut page);
        pages.set(pages_vec);
    }

    cx.render(rsx! {
        Route {
            to: "{cx.props.to}",
            header::header { active_route: cx.props.to }
            &cx.props.content
        }
    })
}

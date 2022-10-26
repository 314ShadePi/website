use dioxus::prelude::*;
use crate::core::state;

#[derive(Props)]
pub struct PageProps<'a> {
    to: &'a str,
    name: &'a str,
    should_be_on_navbar: bool,
    content: Element<'a>
}

pub fn page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    if cx.props.should_be_on_navbar == true {
        let pages = use_atom_ref(&cx, state::PAGES);
        let mut pages_vec = pages.read().to_vec();
        let mut page = vec![state::Page { to: cx.props.to.to_string(), name: cx.props.name.to_string() }];
        page.append(&mut pages_vec);
        pages.set(page);
    }

    cx.render(rsx! {
        Route {
            to: "{cx.props.to}",
            &cx.props.content
        }
    })
}
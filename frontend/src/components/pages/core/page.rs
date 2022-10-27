use crate::core::s_page;

use super::header;
use dioxus::prelude::*;

#[derive(Props)]
pub struct PageProps<'a> {
    to: &'static str,
    name: &'static str,
    should_be_on_navbar: bool,
    content: Element<'a>,
    state: &'a state::Storage<Vec<s_page::Page>>,
}
// https://docs.rs/dioxus/latest/dioxus/prelude/struct.EventHandler.html
pub fn page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    if cx.props.should_be_on_navbar == true {
        let pages = cx.props.state.clone();
        let pages = &*pages.get();
        let pages = &mut pages.clone();
        let mut page = vec![s_page::Page {
            to: cx.props.to,
            name: cx.props.name,
        }];
        pages.append(&mut page);
        cx.props.state.set(pages.to_vec());
    }

    cx.render(rsx! {
        Route {
            to: "{cx.props.to}",
            header::header { active_route: cx.props.to, state: cx.props.state }
            &cx.props.content
        }
    })
}

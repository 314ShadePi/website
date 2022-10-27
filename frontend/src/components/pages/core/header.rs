use dioxus::prelude::*;

use crate::core::s_page;

#[derive(Props)]
pub struct HeaderProps<'a> {
    active_route: &'a str,
    state: &'a state::Storage<Vec<s_page::Page>>
}

// Something is not working here
pub fn header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let pages = &*cx.props.state.get();
    cx.render(rsx! {
        div {
            "Header: active_route = {cx.props.active_route}"
            ul {
                pages.iter().map(|page| rsx! {
                    li { Link { to: "{page.to}", "{page.name}" } }
                })
            }
        }
    })
}

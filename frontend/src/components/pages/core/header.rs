use crate::core::state;
use dioxus::prelude::*;

#[derive(Props)]
pub struct HeaderProps<'a> {
    active_route: &'a str,
}

// Something is not working here
pub fn header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let pages = use_atom_ref(&cx, state::PAGES);
    let pages_vec = pages.read().to_vec();
    cx.render(rsx! {
        div {
            "Header: active_route = {cx.props.active_route}"
            ul {
                pages_vec.iter().map(|page| rsx! {
                    li { Link { to: "{page.to}", "{page.name}" } }
                })
            }
        }
    })
}

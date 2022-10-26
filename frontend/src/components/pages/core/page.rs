use dioxus::prelude::*;

#[derive(Props)]
pub struct PageProps<'a> {
    to: &'a str,
    content: Element<'a>
}

pub fn page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    cx.render(rsx! {
        Route {
            to: "{cx.props.to}",
            &cx.props.content
        }
    })
}
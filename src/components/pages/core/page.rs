use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Page {
    pub to: &'static str,
    pub name: &'static str,
    pub display: bool,
}

#[derive(Props)]
pub struct PageProps<'a> {
    to: &'static str,
    name: &'static str,
    should_be_on_navbar: bool,
    oncreate: EventHandler<'a, Page>,
    children: Element<'a>,
}

pub fn page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    cx.props.oncreate.call(Page {
        to: cx.props.to,
        name: cx.props.name,
        display: cx.props.should_be_on_navbar,
    });

    cx.render(rsx! {
        Route {
            to: "{cx.props.to}",
            main {
                &cx.props.children
            }
        }
    })
}

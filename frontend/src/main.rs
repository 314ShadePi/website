mod components;

use dioxus::prelude::*;

use crate::components::pages::core::page;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Router {
                page::page {to: "/", content: cx.render(rsx! { div { p { "Home" } Link { to: "/about", "About" } }})}
                page::page {to: "/about", content: cx.render(rsx! { div { p { "About" } Link { to: "/", "Home" } }})}
            }
        }
    })
}

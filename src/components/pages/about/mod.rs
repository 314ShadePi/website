use dioxus::prelude::*;

pub fn about(cx: Scope) -> Element {
    let document = gloo::utils::document();
    document.set_title("314ShadePi - About");

    cx.render(rsx! {
        div {
            p { "About" }
            Link { to: "/", "Home" }
        }
    })
}
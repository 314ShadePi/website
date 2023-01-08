use dioxus::prelude::*;

pub fn footer(cx: Scope) -> Element {
    cx.render(rsx! {
        footer {
            div {
                class: "footer-container",
                hr {
                    id: "footer-line"
                }
                div {
                    class: "footer-text-container",
                    span {
                        class: "inner",
                        p {
                            "© 2023 314ShadePi"
                        }
                    }
                }
            }
        }
    })
}

use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Router {
                h1 { style: "color: red;", "LOOOOOOL" }
                Route { to: "/", p { "Home" } Link { to: "/about", "About"} }
                Route { to: "/about", p { "About" } Link { to: "/", "Home"} }
            }
        }
    })
}

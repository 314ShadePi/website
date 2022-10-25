use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Router {
                Route { to: "/home", p { "Home" } Link { to: "/about", "About"} }
                Route { to: "/about", p { "About" } Link { to: "/home", "Home"} }
            }
        }
    })
}

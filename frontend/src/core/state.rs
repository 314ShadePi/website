use dioxus::prelude::*;

#[derive(Clone)]
pub struct Page {
    pub to: &'static str,
    pub name: &'static str,
}

pub static PAGES: AtomRef<Vec<Page>> = |_| {
    vec![Page {
        to: "/",
        name: "Home",
    }]
};

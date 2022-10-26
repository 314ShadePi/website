use dioxus::prelude::*;

#[derive(Clone)]
pub struct Page {
    pub to: String,
    pub name: String
}

pub static PAGES: AtomRef<Vec<Page>> = |_| vec![Page { to: "/".to_string(), name: "Home".to_string()}];
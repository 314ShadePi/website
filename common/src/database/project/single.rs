use super::tags::Tags;
use super::{description_part::DescriptionPart, p_type::Type};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tags: Vec<Tags>,
    pub p_type: Type,
    pub description: Vec<DescriptionPart>,
}

impl Project {
    pub fn render_single(self, cx: Scope) -> Element {
        let binding = self.id;
        let id: &str = binding.as_str();
        cx.render(rsx! {
            div {
                class: "project",
                id: "{id}",
                "LOL"
            }
        })
    }
}

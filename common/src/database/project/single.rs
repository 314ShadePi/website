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
        let binding = self.name;
        let name: &str = binding.as_str();
        let binding = self.p_type.to_string();
        let p_type: &str = binding.as_str();
        let desc = &self.description.clone();
        cx.render(rsx! {
            article {
                class: "project",
                id: "{id}",
                h1 {
                    class: "title p-title",
                    "{name}"
                }
                p {
                    class: "type",
                    "{p_type}"
                }
                p {
                    class: "tags",
                    "tags"
                }
                desc.iter().map(|desc_p| {
                    rsx! {
                        desc_p.clone().render(cx)
                    }
                })
            }
        })
    }
}

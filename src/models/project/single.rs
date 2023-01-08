use crate::traits::to_clink::ToCLink;

use super::download::DownloadLink;
use super::tags::Tags;
use super::{description_part::DescriptionPart, p_type::Type};

use c314_utils::prelude::ToStaticStr;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tags: Vec<Tags>,
    pub p_type: Type,
    pub description: Vec<DescriptionPart>,
    pub downloads: Vec<DownloadLink>,
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
        let binding = self
            .tags
            .iter()
            .map(|tag| tag.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let tags: &str = binding.as_str();
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
                span {
                    class: "download-links",
                        self.downloads.iter().map(|download| {
                            rsx! {
                                span {
                                    class: "link",
                                    download.to_clink().render(cx)
                                    ", "
                                }
                            }
                        })
                    }
                p {
                    class: "p_tags",
                    "Tags: {tags}"
                }
                desc.iter().map(|desc_p| {
                    rsx! {
                        desc_p.clone().render(cx)
                    }
                })
            }
        })
    }

    pub fn render_multiple<'a>(self, cx: Scope<'a>, mut f: impl FnMut(String) + 'a) -> Element<'a> {
        let binding: String = self.id;
        let id: &'static str = binding.to_static_str();
        let binding: String = self.name;
        let name: &'static str = binding.to_static_str();
        let binding: String = self.p_type.to_string();
        let p_type: &'static str = binding.to_static_str();
        let binding: String = self
            .tags
            .iter()
            .map(|tag| tag.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let tags: &'static str = binding.to_static_str();

        cx.render(rsx! {
            article {
                class: "project",
                id: "{id}",
                onclick: move |_| f(id.to_string()),
                h1 {
                    class: "title",
                    "{name}"
                }
                div {
                    class: "info",
                    p {
                        class: "type",
                        "{p_type}"
                    }
                    p {
                        class: "p-tags",
                        "Tags: {tags}"
                    }
                }
            }
        })
    }
}

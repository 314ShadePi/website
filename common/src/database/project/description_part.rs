use serde::{Deserialize, Serialize};
use std::fmt;
use dioxus::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct DescriptionPart {
    /// Determines the contenttype of the subdescription. Used for headings and metadata.
    pub desc_type: DescType,
    /// Set this to true if there is ***Any*** html tags, otherwise they won't be rendered.
    pub is_html: bool,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DescType {
    Usage,
    Installation,
    Faq,
    Other { title: String },
}

impl DescriptionPart {
    fn render(self, cx: Scope) -> Element {
        let binding = self.content;
        let content: &str = binding.as_str();
        let binding = self.desc_type.to_string();
        let binding = binding.replace(" ", "-");
        let title: &str = binding.as_str();
        cx.render(rsx! {
            div {
                class: "subdescription",
                id: "{title}",
                self.desc_type.render(cx)
                if self.is_html == true {
                    rsx! {
                        div {
                            dangerous_inner_html: "{content}"
                        }
                    }
                } else {
                    rsx! {
                        div {
                            "{content}"
                        }
                    }
                }
            }
        })
    }
}

impl DescType {
    fn render(self, cx: Scope) -> Element {
        let binding = self.to_string();
        let title: &str = binding.as_str();
        cx.render(rsx! {
            h2 {
                class: "subtitle",
                "{title}"
            }
        })
    }
}

impl fmt::Display for DescType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DescType::Usage => write!(f, "Usage"),
            DescType::Installation => write!(f, "Installation"),
            DescType::Faq => write!(f, "FAQ"),
            DescType::Other { title } => write!(f, "{}", title),
        }
    }
}

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DescriptionPart {
    /// Determines the content-type of the sub-description. Used for headings and metadata.
    pub desc_type: DescType,
    /// Set this to true if there is ***Any*** html tags, otherwise they won't be rendered.
    pub is_html: bool,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DescType {
    Usage,
    Installation,
    Faq,
    Configuration { c_type: ConfType },
    Other { title: String },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ConfType {
    File,
    Env,
    Cli,
    Other { title: String },
}

impl DescriptionPart {
    pub fn render(self, cx: Scope) -> Element {
        let binding = self.content;
        let content: &str = binding.as_str();
        let binding = self.desc_type.to_string();
        let binding = binding.replace(" ", "-");
        let title: &str = binding.as_str();
        cx.render(rsx! {
            section {
                class: "sub-description",
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
    pub fn render(self, cx: Scope) -> Element {
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
            DescType::Configuration { c_type } => write!(f, "Configuration: {}", c_type),
            DescType::Other { title } => write!(f, "{}", title),
        }
    }
}

impl fmt::Display for ConfType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfType::File => write!(f, "File"),
            ConfType::Env => write!(f, "Environment Variables"),
            ConfType::Cli => write!(f, "CLI Arguments & Options"),
            ConfType::Other { title } => write!(f, "{}", title),
        }
    }
}

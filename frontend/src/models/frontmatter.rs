use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrontMatter {
    title: String,
    author: String,
}

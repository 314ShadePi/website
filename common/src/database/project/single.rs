use super::p_type::Type;
use super::tags::Tags;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tags: Vec<Tags>,
    pub p_type: Type,
}

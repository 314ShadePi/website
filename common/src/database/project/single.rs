use super::{p_type::Type, description_part::DescriptionPart};
use super::tags::Tags;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tags: Vec<Tags>,
    pub p_type: Type,
    pub description: Vec<DescriptionPart>,
}

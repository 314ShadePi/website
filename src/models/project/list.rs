use serde::{Deserialize, Serialize};

use super::single::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectList {
    pub list: Vec<Project>,
}

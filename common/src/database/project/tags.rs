use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Tags {
    Cli,
    Gui,
    Ui,
    Tui,
    Game,
    Lib,
    Bin,
    Util,
    Utils,
    Program,
}

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tags::Cli => write!(f, "CLI"),
            Tags::Gui => write!(f, "GUI"),
            Tags::Ui => write!(f, "UI"),
            Tags::Tui => write!(f, "TUI"),
            Tags::Game => write!(f, "Game"),
            Tags::Lib => write!(f, "Library"),
            Tags::Bin => write!(f, "Binary"),
            Tags::Util => write!(f, "Utility"),
            Tags::Utils => write!(f, "Utilities"),
            Tags::Program => write!(f, "Program"),
        }
    }
}

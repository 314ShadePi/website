use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Type {
    Game { os: Vec<OS>, engine: Engine },
    Desktop { os: Vec<OS>, engine: Engine },
    Web { engine: Engine },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OS {
    Windows { version: Vec<WinVer> },
    Linux,
    Mac,
}

/// Engine refers to both engines, frameworks and libraries
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Engine {
    Unreal { version: UnrealVer },
    Unity,
    Bevy,
    Fyrox,
    Dioxus,
    Yew,
    EguiEframe,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UnrealVer {
    Four,
    Five,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WinVer {
    Ten,
    Eleven,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Game { os, engine } => {
                write!(
                    f,
                    "Game for {} using {}",
                    os.iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    engine.to_string()
                )
            }
            Type::Desktop { os, engine } => {
                write!(
                    f,
                    "Desktop app/program for {} using {}",
                    os.iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    engine.to_string()
                )
            }
            Type::Web { engine } => write!(f, "Website/Webapp using {}", engine.to_string()),
        }
    }
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OS::Windows { version } => {
                write!(
                    f,
                    "Windows {}",
                    version
                        .iter()
                        .map(|ver| ver.to_string())
                        .collect::<Vec<String>>()
                        .join("/")
                )
            }
            OS::Linux => write!(f, "Linux (x86_64-unknown-linux-gnu, probably)"),
            OS::Mac => write!(f, "MacOS"),
        }
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Engine::Unreal { version } => write!(f, "Unreal Engine {}", version.to_string()),
            Engine::Unity => write!(f, "Unity"),
            Engine::Bevy => write!(f, "Bevy"),
            Engine::Fyrox => write!(f, "Fyrox"),
            Engine::Dioxus => write!(f, "Dioxus"),
            Engine::Yew => write!(f, "Yew"),
            Engine::EguiEframe => write!(f, "Egui/Eframe"),
        }
    }
}

impl fmt::Display for UnrealVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnrealVer::Four => write!(f, "4"),
            UnrealVer::Five => write!(f, "5"),
        }
    }
}

impl fmt::Display for WinVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WinVer::Ten => write!(f, "10"),
            WinVer::Eleven => write!(f, "11"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_all() {
        let string = Type::Game {
            os: vec![
                OS::Windows {
                    version: vec![WinVer::Ten, WinVer::Eleven],
                },
                OS::Linux,
                OS::Mac,
            ],
            engine: Engine::Unreal {
                version: UnrealVer::Five,
            },
        }
        .to_string();
        assert_eq!(
            string,
            "Game for Windows 10/11, Linux (x86_64-unknown-linux-gnu, probably), MacOS using Unreal Engine 5".to_string()
        )
    }

    #[test]
    fn desktop_all() {
        let string = Type::Desktop {
            os: vec![
                OS::Windows {
                    version: vec![WinVer::Ten, WinVer::Eleven],
                },
                OS::Linux,
                OS::Mac,
            ],
            engine: Engine::EguiEframe,
        }
        .to_string();
        assert_eq!(string, "Desktop app/program for Windows 10/11, Linux (x86_64-unknown-linux-gnu, probably), MacOS using Egui/Eframe".to_string())
    }

    #[test]
    fn web() {
        let string = Type::Web {
            engine: Engine::Dioxus,
        }
        .to_string();
        assert_eq!(string, "Website/Webapp using Dioxus".to_string())
    }
}

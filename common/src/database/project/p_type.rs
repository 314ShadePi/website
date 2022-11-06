use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Game { os: Vec<OS> },
    Desktop { os: Vec<OS> },
    Web,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OS {
    Windows { version: Vec<WinVer> },
    Linux,
    Mac,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WinVer {
    Ten,
    Eleven,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Game { os } => {
                write!(
                    f,
                    "Game for {}",
                    os.iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Type::Desktop { os } => {
                write!(
                    f,
                    "Desktop app/program for {}",
                    os.iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Type::Web => write!(f, "Website/Webapp"),
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
        }
        .to_string();
        assert_eq!(
            string,
            "Game for Windows 10/11, Linux (x86_64-unknown-linux-gnu, probably), MacOS".to_string()
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
        }
        .to_string();
        assert_eq!(string, "Desktop app/program for Windows 10/11, Linux (x86_64-unknown-linux-gnu, probably), MacOS".to_string())
    }

    #[test]
    fn web() {
        let string = Type::Web.to_string();
        assert_eq!(string, "Website/Webapp".to_string())
    }
}

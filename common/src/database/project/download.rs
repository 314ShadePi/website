use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DownloadLink {
    Steam { link: String },
    GitHub { link: String, release: bool },
    ItchIO { subdomain: String, app: String },
    WindowsD { link: String },
    LinuxD { link: String },
    MacD { link: String },
    None,
}

impl DownloadLink {
    fn get_link(&self) -> String {
        match self {
            DownloadLink::Steam { link } => {
                format!("https://store.steampowered.com/app/{}", link)
            }
            DownloadLink::GitHub { link, release } => {
                if *release == true {
                    format!("https://github.com/{}/releases", link)
                } else {
                    format!("https://github.com/{}", link)
                }
            }
            DownloadLink::ItchIO { subdomain, app } => {
                format!("https://{}.itch.io/{}", subdomain, app)
            }
            DownloadLink::WindowsD { link } => format!("{}", link),
            DownloadLink::LinuxD { link } => format!("{}", link),
            DownloadLink::MacD { link } => format!("{}", link),
            DownloadLink::None => format!(""),
        }
    }
}

use crate::{components::clink::CLink, traits::to_clink::ToCLink};
use serde::{Deserialize, Serialize};

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

impl ToCLink for DownloadLink {
    fn to_clink(&self) -> CLink {
        match self {
            DownloadLink::Steam { link } => CLink(
                format!("https://store.steampowered.com/app/{}", link),
                "Get it on Steam".to_string(),
            ),
            DownloadLink::GitHub { link, release } => CLink(
                if *release == true {
                    format!("https://github.com/{}/releases", link)
                } else {
                    format!("https://github.com/{}", link)
                },
                "Download from GitHub".to_string(),
            ),
            DownloadLink::ItchIO { subdomain, app } => CLink(
                format!("https://{}.itch.io/{}", subdomain, app),
                "Get it on Itch.io".to_string(),
            ),
            DownloadLink::WindowsD { link } => {
                CLink(format!("{}", link), "Download for Windows".to_string())
            }
            DownloadLink::LinuxD { link } => {
                CLink(format!("{}", link), "Download for Linux".to_string())
            }
            DownloadLink::MacD { link } => {
                CLink(format!("{}", link), "Download for MacOS".to_string())
            }
            DownloadLink::None => {
                CLink("".to_string(), "".to_string())
            }
        }
    }
}

use serde::{Deserialize, Serialize};
use url::Url;

const LICENSE_LIST_JSON: &[u8] = include_bytes!("../spdx-licenses/json/licenses.json");

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    package_name: String,
    authors: Authors,
    home: Option<Url>,
    repo: Option<Url>,
    /// Depended on by these pacakges
    depends: Vec<String>,
    /// Depended on by these pacakges
    depended: Vec<String>,
    license: License,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Authors {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum License {
    /// A valid, spdx recognized, license was provided
    Valid {
        name: String,
        id: String,
        category: String,
    },
    /// Not a valid spdx license
    Unrecognized,
    /// No license provided
    None,
}

// TODO: Update license repo at runtime, rather than including it in the binary

impl License {
    /// Checks whether the license is recognized by the Open Source Initiative
    pub fn is_osi(&self) -> bool {
        unimplemented!("to keep up to date this could require either a lot of manual work, or web-scraping, neither of which I am going to implement now")
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PackageManager {
    Cargo,
    // TODO: Add PNPM/NPM/Yarn/Bun/pub, etc.
}

pub mod cargo;

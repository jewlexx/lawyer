use serde::{Deserialize, Serialize};

const LICENSE_LIST_JSON: &[u8] = include_bytes!("../spdx-licenses/json/licenses.json");

lazy_static::lazy_static! {
    pub static ref LICENSE_LIST: List = serde_json::from_slice(LICENSE_LIST_JSON).expect("valid license list");
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub license_list_version: String,
    pub licenses: Vec<License>,
    pub release_date: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub reference: url::Url,
    pub is_deprecated_license_id: bool,
    pub details_url: String,
    pub reference_number: i64,
    pub name: String,
    pub license_id: String,
    pub see_also: Vec<String>,
    pub is_osi_approved: bool,
    pub is_fsf_libre: Option<bool>,
}

#[must_use]
pub enum Id {
    Spdx(String),
    Unknown(String),
}

impl Id {
    pub fn new(id: String) -> Self {
        if LICENSE_LIST
            .licenses
            .iter()
            .any(|license| license.license_id == id)
        {
            Self::Spdx(id)
        } else {
            Self::Unknown(id)
        }
    }
}

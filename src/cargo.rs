use std::{path::Path, str::FromStr};

use cargo_lock::{dependency::Tree, Lockfile};

#[derive(Debug, thiserror::Error)]
pub enum CargoError {
    #[error("Cargo lockfile error {0}")]
    CargoLockError(#[from] cargo_lock::Error),
}

pub type Result<T> = std::result::Result<T, CargoError>;

pub struct CargoLicenses {
    lockfile: Lockfile,
}

impl CargoLicenses {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            lockfile: Lockfile::load(path)?,
        })
    }

    pub fn from_lockfile(lockfile: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            lockfile: Lockfile::from_str(lockfile.as_ref())?,
        })
    }

    pub fn tree(&self) -> Result<Tree> {
        Ok(self.lockfile.dependency_tree()?)
    }
}

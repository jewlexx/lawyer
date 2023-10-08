use std::{ops::Deref, path::Path, rc::Rc, str::FromStr};

use cargo_lock::{dependency::Tree, Lockfile, Package};

#[derive(Debug, thiserror::Error)]
pub enum CargoError {
    #[error("Cargo lockfile error {0}")]
    CargoLockError(#[from] cargo_lock::Error),
    #[error("Package {}:{} is missing a source", .0.name, .0.version)]
    MissingPackageSource(Rc<Package>),
}

pub type Result<T> = std::result::Result<T, CargoError>;

pub struct CargoLicenses {
    lockfile: Lockfile,
}

impl Deref for CargoLicenses {
    type Target = Lockfile;

    fn deref(&self) -> &Self::Target {
        &self.lockfile
    }
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

    pub fn foreach(&self) -> Result<()> {
        for package in &self.packages {
            let package = Rc::new(package.clone());
            let Some(ref pkg_source) = package.source else {
                return Err(CargoError::MissingPackageSource(package.clone()));
            };
        }

        Ok(())
    }
}

use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::Path,
    rc::Rc,
    str::FromStr,
};

use cargo_lock::{dependency::Tree, Dependency, Lockfile, Package};

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

impl DerefMut for CargoLicenses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lockfile
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

    pub fn foreach(mut self) -> Result<()> {
        let mut dependency_map = HashMap::<String, Rc<Vec<Dependency>>>::new();

        for package in &mut self.packages {
            let package_deps = {
                let mut deps: Vec<Dependency> = vec![];

                std::mem::swap(&mut deps, &mut package.dependencies);

                Rc::new(deps)
            };
            let package = Rc::new(package.clone());

            dependency_map.insert(
                format!("{}@{}", package.name, package.version),
                package_deps.clone(),
            );
            let Some(ref pkg_source) = package.source else {
                return Err(CargoError::MissingPackageSource(package.clone()));
            };
        }

        Ok(())
    }
}

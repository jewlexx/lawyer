use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::Path,
    rc::Rc,
    str::FromStr,
};

use cargo_lock::{
    dependency::Tree, Checksum, Dependency, Lockfile, Name, Package, SourceId, Version,
};

use crate::{CratePackageUID, UID};

pub type PackageUID = CratePackageUID<Checksum, Name, Version, SourceId>;

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
        let mut dependency_map = HashMap::<PackageUID, Rc<Vec<Dependency>>>::new();

        for package in &mut self.packages {
            let package_deps = Rc::new(unsafe { crate::grab(&mut package.dependencies) });

            let package = Rc::new(package.clone());

            dependency_map.insert(package.uid(), package_deps.clone());

            let Some(ref pkg_source) = package.source else {
                return Err(CargoError::MissingPackageSource(package.clone()));
            };
        }

        Ok(())
    }
}

impl crate::UID<PackageUID> for Package {
    fn uid(&self) -> PackageUID {
        self.clone().into()
    }
}

impl From<Package> for PackageUID {
    fn from(value: Package) -> Self {
        match value {
            Package {
                checksum: Some(sum),
                ..
            } => PackageUID::Checksum(sum),
            Package {
                name,
                version,
                source: Some(source),
                ..
            } => PackageUID::NameVersionAndSource {
                name,
                version,
                source,
            },
            Package { name, version, .. } => PackageUID::NameAndVersion { name, version },
        }
    }
}

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
pub enum Error {
    #[error("Cargo lockfile error {0}")]
    CargoLockError(#[from] cargo_lock::Error),
    #[error("Package {}:{} is missing a source", .0.name, .0.version)]
    MissingPackageSource(Rc<Package>),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Licenses {
    lockfile: Lockfile,
}

impl Deref for Licenses {
    type Target = Lockfile;

    fn deref(&self) -> &Self::Target {
        &self.lockfile
    }
}

impl DerefMut for Licenses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lockfile
    }
}

pub type DependencyMap =
    HashMap<CratePackageUID<Checksum, Name, Version, SourceId>, Rc<Vec<Dependency>>>;

impl Licenses {
    /// Loads the lockfile from `path`
    ///
    /// # Errors
    /// Loading the lockfile fails for whatever reason. See [`cargo_lock::error::Error`] for more information
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            lockfile: Lockfile::load(path)?,
        })
    }

    /// Loads the lockfile from its contents
    ///
    /// # Errors
    /// Loading the lockfile fails for whatever reason. See [`cargo_lock::error::Error`] for more information
    pub fn from_lockfile(lockfile: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            lockfile: Lockfile::from_str(lockfile.as_ref())?,
        })
    }

    pub fn create_map(mut self) -> Result<DependencyMap> {
        let mut dependency_map = HashMap::<PackageUID, Rc<Vec<Dependency>>>::new();

        for package in &mut self.packages {
            let package_deps = Rc::new(unsafe { crate::grab(&mut package.dependencies) });

            let package = Rc::new(package.clone());

            dependency_map.insert(package.uid(), package_deps.clone());

            let Some(ref pkg_source) = package.source else {
                return Err(Error::MissingPackageSource(package.clone()));
            };
        }

        Ok(dependency_map)
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

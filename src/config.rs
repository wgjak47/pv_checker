use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::remote::get_package_remote;
use crate::version::VersionInfo;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub url: String,
    pub package_type: String,
    pub version_type: String,
}

type PackageConfigs = Vec<PackageConfig>;

pub fn load_package_config(path: &Path) -> Result<PackageConfigs> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let configs: PackageConfigs = serde_yaml::from_str(&s)?;
    Ok(configs)
}

impl PackageConfig {
    pub async fn get_version(&self) -> Result<Box<dyn VersionInfo>> {
        let remote = get_package_remote(
            self.url.clone(),
            self.package_type.clone(),
            self.version_type.clone(),
        )?;
        let version_info = remote.fetch_latest_version().await?;

        Ok(version_info)
    }
}

use std::path::Path;

use derive_more::{Error, From};
use duct::cmd;
use fmt_derive::Display;
use indexmap::IndexMap;
use serde_json::Value;

use crate::types::crate_info::CrateInfo;

pub type DependencyMap = IndexMap<String, CrateInfo>;

pub fn get_dependency_map(manifest_path: &Path) -> Result<DependencyMap, GetDependenciesError> {
    // Execute the cargo metadata command using duct
    let output = cmd!("cargo", "metadata", "--manifest-path", manifest_path, "--format-version", "1").read()?;

    // Parse the JSON output
    let json: Value = serde_json::from_str(&output)?;

    let mut dependencies = IndexMap::new();

    // Iterate through the packages and insert the crate name and manifest path into the IndexMap
    if let Some(packages) = json["packages"].as_array() {
        for package in packages {
            if let Some(name) = package["name"].as_str() {
                if let Some(manifest_path) = package["manifest_path"].as_str() {
                    let path_buf = Path::new(manifest_path)
                        .parent()
                        .unwrap_or_else(|| Path::new(""))
                        .to_path_buf();
                    let key = name.to_string();
                    let existing_dependency_opt = dependencies.insert(key, CrateInfo::new(name.to_string(), path_buf));
                    if let Some(existing_dependency) = existing_dependency_opt {
                        let existing_dependency_name = existing_dependency.name();
                        panic!("Package metadata contains multiple entries for a dependency \"{existing_dependency_name}\"")
                    }
                }
            }
        }
    }

    Ok(dependencies)
}

#[derive(Error, Display, From, Debug)]
pub enum GetDependenciesError {
    IoError(std::io::Error),
    SerdeJsonError(serde_json::Error),
}

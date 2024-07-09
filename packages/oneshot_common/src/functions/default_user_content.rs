use std::path::{Path, PathBuf};

use crate::functions::cargo_toml::cargo_toml_path_buf;
use crate::functions::get_parts_from_maybe_strings::join_message_part_opts;
use crate::functions::readme::readme_path_buf;
use crate::types::source_file::{FromPathBufToXmlIfExists, SourceFile};

pub fn default_user_content_string(package_root_opt: Option<&Path>, workspace_root_opt: Option<&Path>) -> Result<String, FromPathBufToXmlIfExists> {
    default_user_content_vec(package_root_opt, workspace_root_opt).map(join_message_part_opts)
}

pub fn default_user_content_vec(package_root_opt: Option<&Path>, workspace_root_opt: Option<&Path>) -> Result<Vec<Option<String>>, FromPathBufToXmlIfExists> {
    Ok(vec![
        to_xml_via_source_file(package_root_opt, readme_path_buf)?,
        to_xml_via_source_file(package_root_opt, cargo_toml_path_buf)?,
        to_xml_via_source_file(workspace_root_opt, cargo_toml_path_buf)?,
    ])
}

pub fn to_xml_via_source_file(path_opt: Option<&Path>, mapper: impl FnOnce(&Path) -> PathBuf) -> Result<Option<String>, FromPathBufToXmlIfExists> {
    path_opt
        .map(mapper)
        .map(SourceFile::from_path_buf_to_xml_if_exists)
        .transpose()
        .map(Option::flatten)
}

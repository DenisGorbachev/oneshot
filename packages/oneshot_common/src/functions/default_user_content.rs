use std::io;
use std::path::Path;

use crate::functions::cargo_toml::cargo_toml_maybe_if_exists;
use crate::functions::get_parts_from_maybe_strings::collect_maybe_strings;
use crate::functions::implement_file_instruction::implement_file_instruction;
use crate::functions::intro::intro_maybe;
use crate::functions::push_readme_if_exists::readme_maybe_if_exists;
use crate::types::language::Language;

pub fn default_user_content_string(file_path: &Path, language_opt: Option<Language>, package_root_opt: Option<&Path>, workspace_root_opt: Option<&Path>) -> io::Result<String> {
    default_user_content_vec(file_path, language_opt, package_root_opt, workspace_root_opt).map(collect_maybe_strings)
}

pub fn default_user_content_vec(file_path: &Path, language_opt: Option<Language>, package_root_opt: Option<&Path>, workspace_root_opt: Option<&Path>) -> io::Result<Vec<Option<String>>> {
    Ok(vec![
        intro_maybe(language_opt),
        readme_maybe_if_exists(package_root_opt)?,
        cargo_toml_maybe_if_exists(package_root_opt)?,
        cargo_toml_maybe_if_exists(workspace_root_opt)?,
        Some(implement_file_instruction(language_opt, file_path)),
    ])
}

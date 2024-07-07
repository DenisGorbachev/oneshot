use std::io;
use std::path::Path;

use crate::functions::get_parts_from_maybe_strings::collect_maybe_strings;
use crate::functions::implement_file_instruction::implement_file_instruction;
use crate::functions::intro::intro_maybe;
use crate::functions::push_readme_if_exists::readme_maybe_if_exists;
use crate::types::language::Language;

pub fn default_user_content(
    file_path: &Path,
    package_root_opt: Option<&Path>,
    language_opt: Option<Language>,
) -> io::Result<String> {
    let user_content = collect_maybe_strings(vec![
        intro_maybe(language_opt),
        readme_maybe_if_exists(package_root_opt)?,
        Some(implement_file_instruction(language_opt, file_path)),
    ]);
    Ok(user_content)
}

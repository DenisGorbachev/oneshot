use std::path::Path;

use crate::functions::implement_file_instruction::push_implement_file_instruction;
use crate::functions::intro::push_intro_maybe;
use crate::functions::push_readme_if_exists::push_readme_maybe_if_exists;
use crate::types::language::Language;

pub fn push_intro_readme_implement(parts: &mut Vec<String>, path: &Path, language_opt: Option<Language>, package_root_opt: Option<&Path>) -> std::io::Result<()> {
    push_intro_maybe(parts, language_opt);
    push_readme_maybe_if_exists(parts, package_root_opt)?;
    push_implement_file_instruction(parts, language_opt, path);
    Ok(())
}

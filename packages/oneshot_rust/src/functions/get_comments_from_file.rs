use syn::File;

use crate::functions::get_comment_from_attr::get_comment_from_attr;

pub fn get_comments_from_file(file: File) -> Vec<String> {
    file.attrs
        .into_iter()
        .filter_map(get_comment_from_attr)
        .collect()
}

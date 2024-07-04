use syn::File;

pub fn remove_items(file: &mut File) {
    file.items = vec![]
}

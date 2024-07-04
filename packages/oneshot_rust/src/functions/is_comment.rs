use syn::Attribute;

pub fn is_comment(attr: &Attribute) -> bool {
    attr.path().is_ident("doc")
}

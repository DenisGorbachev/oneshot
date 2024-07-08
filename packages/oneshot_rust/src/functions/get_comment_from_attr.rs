use quote::ToTokens;
use syn::{Attribute, Meta};

pub fn get_comment_from_attr(attr: Attribute) -> Option<String> {
    match attr.meta {
        Meta::NameValue(name_value) if name_value.path.is_ident("doc") => name_value.value.to_token_stream().to_string().into(),
        _ => None,
    }
}

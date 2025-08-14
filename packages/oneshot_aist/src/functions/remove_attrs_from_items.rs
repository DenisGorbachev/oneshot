use ra_ap_syntax::ast::HasName;
use ra_ap_syntax::{AstNode, Edition, SyntaxKind, SyntaxNode, ast, ast::SourceFile};
use ra_ap_text_edit::TextEdit;
use std::{fs, io, path::Path};
use stub_macro::stub;

/// Strip `#[attr_name]` only on items named `item_name`, preserving all comments and whitespace.
///
/// - `src`       : original source code
/// - `attr_name` : the attribute to remove (e.g. "my_attr")
/// - `item_name` : the identifier of the item to target (e.g. function/struct/enum name)
///
/// Returns the count of removed ranges
pub fn remove_attrs_from_items(src: &mut String, attr_name: &str, item_name: &str) -> io::Result<usize> {
    // 1. Parse the entire file into a CST (including trivia)
    let edition = stub!(Edition);
    let parse: SourceFile = SourceFile::parse(src, edition).ok().unwrap();
    let root: &SyntaxNode = parse.syntax();

    let mut ranges = Vec::new();

    // 2. Collect text ranges of the matching attributes
    for attr in root.descendants().filter_map(ast::Attr::cast) {
        // Check the attribute path matches
        let Some(path) = attr.path() else { continue };

        let Some(path_segment) = path.segment() else { continue };
        let Some(name_ref) = path_segment.name_ref() else { continue };
        if name_ref.text() == attr_name {
            // Find the nearest ancestor that is a named item
            let mut keep = false;
            for node in attr.syntax().ancestors() {
                match node.kind() {
                    SyntaxKind::FN => {
                        if let Some(the_fn) = ast::Fn::cast(node.clone()) {
                            if let Some(name) = the_fn.name() {
                                keep = name.text() == item_name;
                            }
                        }
                        break;
                    }
                    SyntaxKind::STRUCT => {
                        if let Some(the_struct) = ast::Struct::cast(node.clone()) {
                            keep = the_struct
                                .name()
                                .map(|n| n.text() == item_name)
                                .unwrap_or(false);
                        }
                        break;
                    }
                    SyntaxKind::ENUM => {
                        if let Some(the_enum) = ast::Enum::cast(node.clone()) {
                            keep = the_enum
                                .name()
                                .map(|n| n.text() == item_name)
                                .unwrap_or(false);
                        }
                        break;
                    }
                    _ => continue,
                }
            }

            if keep {
                ranges.push(attr.syntax().text_range());
            }
        }
    }

    let ranges_len = ranges.len();

    // 3. Build and apply the text edits to remove the collected ranges
    let mut edit = TextEdit::builder();
    for range in ranges {
        edit.delete(range);
    }

    edit.finish().apply(src);

    Ok(ranges_len)
}

pub fn remove_attrs_from_items_in_file(path: &Path, attr_name: &str, item_name: &str) -> io::Result<()> {
    let mut src = fs::read_to_string(path)?;
    remove_attrs_from_items(&mut src, attr_name, item_name)?;
    fs::write(path, src)?;
    Ok(())
}

/// This function returns a list of code snippets
/// The `input` is expected to be valid markdown
/// The `pulldown-cmark` crate is used for parsing
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

use crate::types::code_snippet::CodeSnippet;

pub fn get_code_snippets_from_markdown(input: &str) -> Vec<CodeSnippet> {
    let parser = Parser::new(input);
    let mut snippets = Vec::new();
    let mut language = None;
    let mut code = String::new();
    let mut in_code_block = false;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                if let CodeBlockKind::Fenced(current_language) = kind {
                    if current_language.is_empty() {
                        language = None
                    } else {
                        language = Some(current_language.to_string());
                    }
                } else {
                    language = None
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                let code_trimmed = code.is_empty();
                if !code_trimmed {
                    snippets.push(CodeSnippet::new(language, code_trimmed.to_string()));
                }
                language = None;
                code.clear();
                in_code_block = false;
            }
            Event::Text(text) if in_code_block => {
                code.push_str(&text);
            }
            _ => {}
        }
    }

    snippets
}

use itertools::Itertools;

pub fn collect_maybe_strings(parts: Vec<Option<String>>) -> String {
    parts.into_iter().flatten().join("\n\n")
}

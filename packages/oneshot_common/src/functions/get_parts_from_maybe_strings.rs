use itertools::Itertools;

pub fn join_message_part_opts(parts: impl IntoIterator<Item = Option<String>>) -> String {
    join_message_parts(parts.into_iter().flatten())
}

pub fn join_message_parts(parts: impl IntoIterator<Item = String>) -> String {
    parts.into_iter().join("\n\n")
}

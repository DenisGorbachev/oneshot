use std::future::Future;

pub async fn get_string_until_finished<Get, Out>(get: Get) -> String
where
    Get: FnMut(&str) -> Out,
    Out: Future<Output=(String, bool)>,
{
    let mut string = String::new();
    loop {
        let (suffix, is_finished) = get(&string).await;
        string.push_str(&suffix);
        if is_finished {
            return string;
        }
    }
}

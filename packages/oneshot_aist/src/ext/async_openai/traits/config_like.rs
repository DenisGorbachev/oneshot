use async_openai::config::Config;

/// Use `ConfigLike` trait instead of `Config` trait so that we could use the `Config` name for a type, not a trait
/// Example: `pub fn foo<Config: ConfigLike>(config: Config)`
pub trait ConfigLike: Config {}

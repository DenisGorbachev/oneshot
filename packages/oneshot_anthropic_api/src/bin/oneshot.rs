use oneshot_common::functions::append_readme::append_readme;
use oneshot_common::functions::intro::intro_raw;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cwd = env!("CARGO_MANIFEST_DIR");
    let mut message = intro_raw("Rust", "package");
    append_readme(cwd, &mut message)?;
    Ok(())
}

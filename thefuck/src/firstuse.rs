use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    thefuck::entrypoints::firstuse::main().await
}

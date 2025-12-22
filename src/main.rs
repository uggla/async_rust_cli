use std::env;

use async_rust_cli::{api_check, run};

fn main() -> anyhow::Result<()> {
    // Load env and API key
    let _ = dotenvy::dotenv();
    let api_key = env::var("SNCF_API_KEY")?;

    run()?;

    dbg!(&api_key);
    api_check(api_key)?;

    Ok(())
}

use std::env;

use async_rust_cli::{api_check, run};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> anyhow::Result<()> {
    // Setup logging
    let file_appender = tracing_appender::rolling::daily("logs", "ratatai.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(non_blocking_appender)
                .with_ansi(false),
        )
        .init();

    tracing::info!("Application starting");
    // Load env and API key
    let _ = dotenvy::dotenv();
    let api_key = env::var("SNCF_API_KEY")?;

    run()?;

    dbg!(&api_key);
    api_check(api_key)?;

    tracing::info!("Application ending");
    Ok(())
}

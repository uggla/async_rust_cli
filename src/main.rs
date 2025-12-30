use std::env;

use async_rust_cli::{APPNAME, api_check, run};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    let file_appender = tracing_appender::rolling::daily("logs", format!("{}.log", APPNAME));
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

    let _ = dotenvy::dotenv();
    let api_key = env::var("SNCF_API_KEY")?;
    api_check(api_key.clone())?;
    run(api_key).await?;

    tracing::info!("Application ending");
    Ok(())
}

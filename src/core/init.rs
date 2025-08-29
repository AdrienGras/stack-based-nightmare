use color_eyre::eyre::Result;
use tracing::debug;
use tracing_subscriber::EnvFilter;

#[tracing::instrument(skip_all, level = "trace")]
pub async fn init() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env()) // autorise TRACE
        .with_line_number(false)
        .with_thread_ids(false)
        .with_target(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    debug!("Tracing initialized");

    color_eyre::install()?;

    debug!("Color eyre initialized");

    Ok(())
}

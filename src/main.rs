#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            &config::get().tracing.filter, // "sea_orm=info,sqlx::query=info,debug",
        ))
        .init();
    db::init_db().await?;

    tokio::spawn(async move {
        let result = collector::run().await;
        if let Err(e) = result {
            tracing::error!("Collector encountered an error: {:?}", e);
        }
    });
    tokio::spawn(async move {
        let result = api::run().await;
        if let Err(e) = result {
            tracing::error!("API encountered an error: {:?}", e);
        }
    });

    tokio::signal::ctrl_c().await?;
    Ok(())
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            &config::get().tracing.filter, // "sea_orm=info,sqlx::query=info,debug",
        ))
        .init();
    db::init_db().await?;

    tokio::spawn(collector::run());
    tokio::spawn(api::run());

    tokio::signal::ctrl_c().await?;
    Ok(())
}

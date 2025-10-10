pub async fn fetch() -> anyhow::Result<(String, String)> {
    let mirror = &config::get().collector.onair.mirror;
    let fetcher = fetcher::get_onair();
    let data: String = fetcher.get(mirror).send().await?.text().await?;
    let hash = md5::compute(&data);
    let hash = format!("{:x}", hash);
    Ok((hash, data))
}

pub async fn refresh() -> anyhow::Result<()> {
    let (hash, data) = fetch().await?;
    if !service::onair::diff_hash(&hash).await? {
        tracing::debug!("OnAir data not changed, skip");
        return Ok(());
    }
    let items = parser::onair::parse(&data)?;
    service::onair::flush(hash, items).await?;
    Ok(())
}

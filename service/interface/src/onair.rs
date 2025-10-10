use crate::collection;
use db::TransactionTrait;
use model::common::onair::{BangumiItemMap, SubjectIds};
pub async fn diff_hash(hash: &str) -> anyhow::Result<bool> {
    collection::kv::OnAir::get(db::get_db())
        .await
        .map(|v| v.diff_hash(&hash))
}

pub async fn flush(hash: String, items: BangumiItemMap) -> anyhow::Result<()> {
    let hash = hash.to_string();
    db::get_db()
        .transaction::<_, _, anyhow::Error>(|txn| {
            Box::pin(async move {
                collection::onair::upsert_many(txn, items).await?;
                collection::kv::OnAir::update(txn, &hash).await?;
                Ok(())
            })
        })
        .await?;

    Ok(())
}

pub async fn find_by_subject_ids(ids: &SubjectIds) -> anyhow::Result<BangumiItemMap> {
    collection::onair::find_by_subject_ids(db::get_db(), ids).await
}

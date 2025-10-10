use db::prelude::*;
use db::sea_query::OnConflict;
use model::common::onair::{BangumiItemMap, SubjectIds};
use model::entity::on_air::{ActiveModel, Column, Entity};

pub async fn upsert_many(db: &impl ConnectionTrait, items: BangumiItemMap) -> anyhow::Result<()> {
    let models = items
        .iter()
        .map(|(id, item)| ActiveModel {
            subject: db::Set(*id),
            data: db::Set(item.to_owned()),
        })
        .collect::<Vec<_>>();
    Entity::insert_many(models)
        .on_conflict(
            OnConflict::column(Column::Subject)
                .update_column(Column::Data)
                .to_owned(),
        )
        .exec(db)
        .await?;
    Ok(())
}

pub async fn find_by_subject_ids(
    db: &impl ConnectionTrait,
    ids: &SubjectIds,
) -> anyhow::Result<BangumiItemMap> {
    let query = Entity::find().filter(Column::Subject.is_in(ids.clone()));
    let items = query.all(db).await?;
    let items: BangumiItemMap = items
        .into_iter()
        .map(|item| (item.subject, item.data))
        .collect();
    Ok(items)
}

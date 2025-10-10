use crate::collection;
use model::common::user::{InitUser, Names, NamesUpdate, Uid};
use model::entity::user::Model;

pub async fn find_by_uid(uid: Uid) -> anyhow::Result<Option<Model>> {
    collection::user::find_by_uid(db::get_db(), uid).await
}

pub async fn init_user(init: InitUser) -> anyhow::Result<Model> {
    collection::user::init_user(db::get_db(), init).await
}

pub async fn update_name_history(uid: Uid, update: NamesUpdate) -> anyhow::Result<Names> {
    collection::user::update_name_history(db::get_db(), uid, update).await
}

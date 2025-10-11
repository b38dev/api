use db::{ActiveModelTrait, Set, TryIntoModel};
use db::{Condition, prelude::*};
use model::common::user::{Extra, InitUser, NameHistory, NamesUpdate, Uid};
use model::entity::user::{ActiveModel, Column, Entity, Model};

pub async fn find_by_uid(db: &impl ConnectionTrait, uid: Uid) -> anyhow::Result<Option<Model>> {
    let cond = match uid.clone() {
        Uid::Nid(nid) => Column::Nid.eq(nid),
        Uid::Sid(sid) => Column::Sid.eq(sid),
    };
    let cond = Condition::all().add(cond);
    let user = Entity::find().filter(cond).one(db).await;
    if user.is_err() {
        tracing::error!(
            "Failed to find user by uid {}: {:?}",
            uid.to_string(),
            &user.as_ref()
        );
    }
    Ok(user?)
}

async fn insert_user(db: &impl ConnectionTrait, init: InitUser) -> anyhow::Result<Model> {
    let extra = Extra {
        name_history: init.names_update.map(|nu| NameHistory {
            update_at: chrono::Utc::now(),
            key_point: nu.key_point,
            names: nu.names,
        }),
        collections: init.collections,
    };
    let user = ActiveModel {
        id: Set(Uuid::new_v4()),
        nid: Set(init.nid),
        sid: Set(init.sid),
        name: Set(init.name),
        join_time: Set(init.join_time),
        last_active: Set(init.last_active),
        state: Set(init.state),
        extra: Set(extra),
        update_at: Set(chrono::Utc::now()),
    };
    let result = Entity::insert(user).exec(db).await?;
    let inserted = Entity::find()
        .filter(Column::Id.eq(result.last_insert_id))
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch inserted user"))?;
    Ok(inserted)
}

async fn update_user(
    db: &impl ConnectionTrait,
    init: InitUser,
    existing: Model,
) -> anyhow::Result<Model> {
    // update fields from init
    let mut extra = existing.extra.clone();
    let mut am: ActiveModel = existing.into();
    if let Some(nid) = init.nid {
        am.nid = Set(Some(nid));
    }
    if let Some(sid) = init.sid {
        am.sid = Set(Some(sid));
    }
    am.name = Set(init.name);
    am.state = Set(init.state);
    am.last_active = Set(init.last_active);
    match (init.collections, init.names_update) {
        (None, None) => {}
        (Some(collections), None) => {
            am.extra = Set(extra.update_collections(collections).to_owned());
        }
        (None, Some(names_update)) => {
            am.extra = Set(extra.update_name_history(names_update).to_owned());
        }
        (Some(collections), Some(names_update)) => {
            extra.update_collections(collections);
            extra.update_name_history(names_update);
            am.extra = Set(extra);
        }
    }
    am.update_at = Set(chrono::Utc::now());
    am.clone().save(db).await?;
    Ok(am.try_into_model()?)
}

pub async fn upsert_user(db: &impl ConnectionTrait, init: InitUser) -> anyhow::Result<Model> {
    // determine if a user exists by nid or sid (require at least one)
    let nid_opt = init.nid.clone();
    let sid_opt = init.sid.clone();
    let cond = match (nid_opt, sid_opt) {
        (None, None) => {
            return Err(anyhow::anyhow!(
                "Either nid or sid must be provided for upsert"
            ));
        }
        (Some(nid), None) => Condition::all().add(Column::Nid.eq(nid)),
        (None, Some(sid)) => Condition::all().add(Column::Sid.eq(sid)),
        (Some(nid), Some(sid)) => Condition::any()
            .add(Column::Nid.eq(nid))
            .add(Column::Sid.eq(sid)),
    };
    if let Some(existing) = Entity::find().filter(cond).one(db).await? {
        update_user(db, init, existing).await
    } else {
        insert_user(db, init).await
    }
}

pub async fn update_name_history(
    db: &impl ConnectionTrait,
    uid: Uid,
    names_update: NamesUpdate,
) -> anyhow::Result<Model> {
    let user = find_by_uid(db, uid).await?;
    let user = user.ok_or(anyhow::anyhow!("User not found"))?;
    let mut extra = user.extra.clone();
    let extra = extra.update_name_history(names_update).to_owned();
    let mut user: ActiveModel = user.into();
    user.extra = Set(extra);
    user.clone().save(db).await?;
    Ok(user.try_into_model()?)
}

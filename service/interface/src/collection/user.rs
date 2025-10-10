use db::{ActiveModelTrait, Set};
use db::{Condition, prelude::*};
use model::common::user::{Extra, InitUser, NameHistory, Names, NamesUpdate, Uid};
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

pub async fn init_user(db: &impl ConnectionTrait, init: InitUser) -> anyhow::Result<Model> {
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
        join_time: Set(init.join_time.map(|t| t.naive_utc())),
        state: Set(init.state),
        extra: Set(extra),
        update_at: Set(chrono::Utc::now().naive_utc()),
    };
    let result = Entity::insert(user).exec(db).await?;
    let inserted = Entity::find()
        .filter(Column::Id.eq(result.last_insert_id))
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch inserted user"))?;
    Ok(inserted)
}

pub async fn update_name_history(
    db: &impl ConnectionTrait,
    uid: Uid,
    NamesUpdate { key_point, names }: NamesUpdate,
) -> anyhow::Result<Names> {
    let user = find_by_uid(db, uid).await?;
    let user = user.ok_or(anyhow::anyhow!("User not found"))?;
    let mut extra = user.extra.clone();
    let update_at = chrono::Utc::now();
    let key_point = key_point.to_string();
    if let Some(nh) = &mut extra.name_history {
        nh.update_at = update_at;
        nh.key_point = key_point;
        nh.names.extend(names);
    } else {
        extra.name_history = Some(NameHistory {
            update_at,
            key_point,
            names,
        });
    }
    let mut user: ActiveModel = user.into();
    let names = extra.name_history.as_ref().unwrap().names.clone();
    user.extra = Set(extra);
    user.save(db).await?;
    Ok(names)
}

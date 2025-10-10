use db::prelude::*;
use db::sea_query::OnConflict;
use model::entity::key_value::{ActiveModel, Column, Entity};
use serde_json::json;
use std::str::FromStr;

pub async fn get(db: &impl ConnectionTrait, key: &str) -> anyhow::Result<Option<Json>> {
    let rec = Entity::find_by_id(key.to_string())
        .one(db)
        .await?
        .map(|kv| kv.value);
    Ok(rec)
}
pub async fn set(db: &impl ConnectionTrait, key: &str, value: &Json) -> anyhow::Result<()> {
    let model = ActiveModel {
        key: db::Set(key.to_string()),
        value: db::Set(value.clone()),
    };
    Entity::insert(model)
        .on_conflict(
            OnConflict::column(Column::Key)
                .update_column(Column::Value)
                .to_owned(),
        )
        .exec(db)
        .await?;
    Ok(())
}

pub struct OnAir {
    pub hash: Option<String>,
    pub update_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Json> for OnAir {
    fn from(value: Json) -> Self {
        let hash = value
            .get("hash")
            .and_then(|h| h.as_str())
            .map(|s| s.to_string());

        let update_at = value
            .get("update_at")
            .and_then(|t| t.as_str())
            .and_then(|s| DateTimeUtc::from_str(s).ok());
        Self { hash, update_at }
    }
}

impl Default for OnAir {
    fn default() -> Self {
        Self {
            hash: None,
            update_at: None,
        }
    }
}

impl OnAir {
    pub async fn get(db: &impl ConnectionTrait) -> anyhow::Result<Self> {
        let rec = get(db, "onair")
            .await?
            .map_or_else(|| Self::default(), |j| j.into());
        Ok(rec)
    }

    pub async fn set(
        db: &impl ConnectionTrait,
        hash: &str,
        update_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<()> {
        let j = json!({
            "hash": hash,
            "update_at": update_at.to_rfc3339(),
        });
        set(db, "onair", &j).await?;
        Ok(())
    }

    pub async fn update(db: &impl ConnectionTrait, hash: &str) -> anyhow::Result<()> {
        Self::set(db, hash, chrono::Utc::now()).await
    }

    pub fn diff_hash(&self, hash: &str) -> bool {
        match &self.hash {
            Some(old_hash) => old_hash != hash,
            None => true,
        }
    }
}

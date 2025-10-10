pub mod bangumi_data;

pub use bangumi_data::{BangumiData, BangumiItem, SiteList as Site};
use std::collections::{HashMap, HashSet};

pub use crate::entity::on_air::SubjectId;
pub type BangumiItemMap = HashMap<SubjectId, BangumiItem>;

impl From<BangumiData> for BangumiItemMap {
    fn from(data: BangumiData) -> Self {
        data.items
            .iter()
            .filter_map(|item| {
                for bangumi_data::Site { site, id, .. } in &item.sites {
                    if !site.eq(&Site::Bangumi) {
                        continue;
                    }
                    if let Some(id) = &id {
                        if let Ok(id) = id.parse() {
                            return Some((id, item.clone()));
                        } else {
                            tracing::warn!("Invalid Bangumi site id for item: {item:?}, id: {id}");
                            return None;
                        }
                    } else {
                        tracing::warn!("No Bangumi site id for item: {item:?}");
                        return None;
                    }
                }
                tracing::debug!("No Bangumi site for item: {:?}", item);
                None
            })
            .collect()
    }
}

pub type SubjectIds = HashSet<SubjectId>;

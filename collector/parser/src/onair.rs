use model::common::onair::{BangumiData, BangumiItemMap};
use std::str::FromStr;

pub fn parse(data: &str) -> anyhow::Result<BangumiItemMap> {
    let data = BangumiItemMap::from(BangumiData::from_str(data)?);
    Ok(data)
}

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub site: SiteList,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Tv,
    Web,
    Movie,
    Ova,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SiteType {
    Onair,
    Info,
    Resource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Language {
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
    #[serde(rename = "en")]
    En,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct BangumiItem {
    pub title: String,
    pub title_translate: HashMap<String, Vec<String>>,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub lang: Language,
    pub official_site: String,
    pub begin: String,
    #[serde(default)]
    pub broadcast: String,
    pub end: String,
    #[serde(default)]
    pub comment: String,
    pub sites: Vec<Site>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BangumiData {
    pub site_meta: HashMap<SiteList, SiteMeta>,
    pub items: Vec<BangumiItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SiteMeta {
    pub title: String,
    pub url_template: String,
    #[serde(default)]
    pub regions: Vec<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_type: Option<SiteType>,
}

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SiteList {
    Bangumi,
    Acfun,
    Bilibili,
    #[serde(rename = "bilibili_hk_mo_tw")]
    BilibiliHkMoTw,
    #[serde(rename = "bilibili_hk_mo")]
    BilibiliHkMo,
    #[serde(rename = "bilibili_tw")]
    BilibiliTw,
    Youku,
    Qq,
    Iqiyi,
    Letv,
    Mgtv,
    Nicovideo,
    Netflix,
    Gamer,
    #[serde(rename = "gamer_hk")]
    GamerHk,
    #[serde(rename = "muse_hk")]
    MuseHk,
    #[serde(rename = "muse_tw")]
    MuseTw,
    #[serde(rename = "ani_one")]
    AniOne,
    #[serde(rename = "ani_one_asia")]
    AniOneAsia,
    Viu,
    Mytv,
    Disneyplus,
    Abema,
    Unext,
    Tropics,
    Prime,
    Dmhy,
    Mikan,
    #[serde(rename = "bangumi_moe")]
    BangumiMoe,
    #[serde(untagged)]
    Other(String),
}

impl FromStr for BangumiData {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let data = serde_json::from_str::<Self>(s)?;
        Ok(data)
    }
}

impl TryFrom<String> for BangumiData {
    type Error = anyhow::Error;
    fn try_from(s: String) -> anyhow::Result<Self> {
        Self::from_str(&s)
    }
}

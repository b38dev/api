use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, Serialize)]
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

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Tv,
    Web,
    Movie,
    Ova,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SiteType {
    Onair,
    Info,
    Resource,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
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

#[derive(Deserialize, Debug, Clone, Serialize)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BangumiData {
    pub site_meta: HashMap<SiteList, SiteMeta>,
    pub items: Vec<BangumiItem>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
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

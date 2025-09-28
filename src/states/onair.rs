use crate::config::OnAirConfig;
use crate::error::AppError;
use crate::models::onair::{BangumiData, BangumiItem, SiteList};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use tokio::sync::RwLock;
use tracing::warn;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    items: HashMap<String, BangumiItem>,
    hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    cache: Cache,
}

impl Data {
    fn new() -> Self {
        Self {
            cache: Cache {
                items: HashMap::new(),
                hash: "".to_string(),
            },
        }
    }

    fn get(&self, id: &str) -> Option<&BangumiItem> {
        self.cache.items.get(id)
    }

    fn get_items(&self) -> &HashMap<String, BangumiItem> {
        &self.cache.items
    }

    fn get_hash(&self) -> &str {
        &self.cache.hash
    }

    fn update(&mut self, cache: Cache) {
        self.cache = cache;
    }
}

#[derive(Debug)]
pub struct OnAir {
    pub mirror: String,
    pub cache_path: String,
    pub client: reqwest::Client,
    pub data: RwLock<Data>,
}

impl OnAir {
    pub fn new(config: &OnAirConfig) -> Self {
        let mut client_builder = reqwest::Client::builder();
        if let Some(proxy) = &config.proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy).unwrap());
        }
        Self {
            mirror: config.mirror.clone(),
            cache_path: config.cache_path.clone(),
            client: client_builder.build().unwrap(),
            data: RwLock::new(Data::new()),
        }
    }

    pub async fn get(&self, id: &str) -> Option<BangumiItem> {
        if let Some(data) = self.data.read().await.get(id) {
            Some(data.to_owned())
        } else {
            None
        }
    }

    pub async fn get_items(&self) -> HashMap<String, BangumiItem> {
        self.data.read().await.get_items().to_owned()
    }

    pub async fn init(&self) -> Result<(), AppError> {
        if let Err(_) = self.load_cache().await {
            self.refresh().await?
        }
        Ok(())
    }

    pub async fn load_cache(&self) -> Result<(), AppError> {
        let content = fs::read_to_string(&self.cache_path).await?;
        let cache: Cache = serde_json::from_str(&content).map_err(|e| AppError::from(e))?;
        self.data.write().await.update(cache);
        Ok(())
    }

    pub async fn refresh(&self) -> Result<(), AppError> {
        let bangumi_data: String = self.client.get(&self.mirror).send().await?.text().await?;
        let hash = md5::compute(&bangumi_data);
        let hash = format!("{:x}", hash);

        if self.data.read().await.get_hash().eq(&hash) {
            return Ok(());
        }
        let mut items = HashMap::new();
        serde_json::from_str::<BangumiData>(&bangumi_data)?
            .items
            .iter()
            .for_each(|item| {
                for site in item.sites.iter() {
                    if let SiteList::Bangumi = site.site {
                        if let Some(id) = &site.id {
                            items.insert(id.clone(), item.clone());
                        }
                        return;
                    }
                }
            });
        let cache = Cache { items, hash };
        let json_string = serde_json::to_string(&cache)?;
        self.data.write().await.update(cache);

        let path = Path::new(&self.cache_path);
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                warn!("创建 cache 目录失败: {}", e);
            }
        }
        if let Err(e) = fs::write(path, json_string).await {
            warn!("写入文件缓存失败: {}", e);
        }
        Ok(())
    }
}

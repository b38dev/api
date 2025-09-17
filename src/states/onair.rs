use crate::error::AppError;
use crate::models::onair::{BangumiData, BangumiItem, SiteList};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tracing::warn;

pub const DAILY_REFRESH_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 24 小时

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub items: HashMap<String, BangumiItem>,
    pub hash: String,
}

impl Data {
    fn get(&self, id: &str) -> Option<&BangumiItem> {
        self.items.get(id)
    }
}

#[derive(Debug)]
pub struct OnAir {
    pub mirror: String,
    // "https://cdn.jsdelivr.net/gh/bangumi-data/bangumi-data@latest/dist/data.json"
    pub cache_path: String,
    // "cache/data.json"
    pub client: reqwest::Client,
    pub data: Option<Arc<Data>>,
}

impl OnAir {
    pub fn new(mirror: &str, cache_path: &str, proxy: Option<String>) -> Self {
        let mut client_builder = reqwest::Client::builder();
        if let Some(proxy) = proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::http(proxy).unwrap());
        }
        Self {
            mirror: mirror.to_string(),
            cache_path: cache_path.to_string(),
            client: client_builder.build().unwrap(),
            data: None,
        }
    }

    pub fn get(&self, id: &str) -> Option<&BangumiItem> {
        if let Some(data) = &self.data {
            data.get(id)
        } else {
            None
        }
    }

    pub async fn init(&mut self) -> Result<(), AppError> {
        if let Err(_) = self.load_cache().await {
            self.refresh().await?
        }
        Ok(())
    }

    pub async fn load_cache(&mut self) -> Result<(), AppError> {
        let content = fs::read_to_string(&self.cache_path).await?;
        let data: Data = serde_json::from_str(&content).map_err(|e| AppError::from(e))?;
        self.data = Some(Arc::new(data));
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<(), AppError> {
        let bangumi_data: String = self
            .client
            .get(&self.mirror)
            .timeout(Duration::from_secs(15))
            .send()
            .await?
            .text()
            .await?;
        let hash = md5::compute(&bangumi_data);
        let hash = format!("{:x}", hash);
        if let Some(data) = &self.data {
            if data.hash.eq(&hash) {
                return Ok(());
            }
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
        let data = Data { items, hash };
        let json_string = serde_json::to_string(&data)?;

        let path = Path::new(&self.cache_path);
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                warn!("创建 cache 目录失败: {}", e);
            }
        }
        if let Err(e) = fs::write(path, json_string).await {
            warn!("写入文件缓存失败: {}", e);
        }
        self.data = Some(Arc::new(data));
        Ok(())
    }
}

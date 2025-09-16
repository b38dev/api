use crate::error::AppError;
use crate::models::{BangumiData, BangumiItem};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::sync::RwLock;
use tracing::warn;

// 定义缓存的有效期
pub const DAILY_REFRESH_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 24 小时

/// 缓存结构
#[derive(Debug)]
pub struct Cache {
    pub data: Arc<HashMap<String, BangumiItem>>,
    pub last_updated: Option<Instant>,
}

impl Cache {
    fn new() -> Self {
        Self {
            data: Arc::new(HashMap::new()),
            last_updated: None,
        }
    }
}

/// 应用的共享状态
#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub subject_map: Arc<RwLock<Cache>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            subject_map: Arc::new(RwLock::new(Cache::new())),
        }
    }
}

pub async fn load_map_from_file() -> Result<HashMap<String, BangumiItem>, AppError> {
    let content = fs::read_to_string("cache/data.json").await?;
    let data: HashMap<String, BangumiItem> =
        serde_json::from_str(&content).map_err(|e| AppError::from(e))?;
    Ok(data)
}

pub async fn load_map_from_net(state: AppState) -> Result<HashMap<String, BangumiItem>, AppError> {
    let bangumi_data: BangumiData = state
        .client
        .get("https://cdn.jsdelivr.net/gh/bangumi-data/bangumi-data@latest/dist/data.json")
        .send()
        .await?
        .json()
        .await?;

    let mut map = HashMap::new();
    bangumi_data.items.iter().for_each(|item| {
        for site in item.sites.iter() {
            if let "bangumi" = site.site.as_ref() {
                if let Some(id) = &site.id {
                    map.insert(id.clone(), item.clone());
                }
                return;
            }
        }
    });

    let json_string = serde_json::to_string(&map)?;

    if let Err(e) = fs::create_dir_all("cache").await {
        warn!("创建 cache 目录失败: {}", e);
    }

    if let Err(e) = fs::write("cache/data.json", json_string).await {
        warn!("写入文件缓存失败: {}", e);
    }

    Ok(map)
}

/// 从网络获取数据并刷新缓存的函数
pub async fn refresh_cache(state: AppState) -> Result<(), AppError> {
    let map = if let None = state.subject_map.read().await.last_updated {
        load_map_from_file()
            .await
            .or(load_map_from_net(state.clone()).await)?
    } else {
        load_map_from_net(state.clone()).await?
    };

    let mut cache = state.subject_map.write().await;
    let data_arc = Arc::new(map);
    cache.data = data_arc.clone();
    cache.last_updated = Some(Instant::now());

    Ok(())
}

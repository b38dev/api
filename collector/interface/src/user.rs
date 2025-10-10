use crate::common::TaskQueue;
use std::sync::LazyLock;

use model::common::user::{InitUser, NamesUpdate, Uid};
use model::prelude::User;

static QUEUE: LazyLock<TaskQueue<Uid, User>> = LazyLock::new(|| TaskQueue::new(10));

pub struct Compass {
    uid: Uid,
}

impl Compass {
    pub fn new(uid: Uid) -> Self {
        Self { uid }
    }

    pub fn with_origin(path: &str) -> String {
        format!("{}/{path}", config::get().collector.user.random_origin()).to_string()
    }

    pub fn home(&self) -> String {
        Self::with_origin(&format!("user/{}", self.uid.to_string()))
    }

    pub fn timeline_say_with_page(&self, page: usize) -> String {
        Self::with_origin(&format!(
            "user/{}/timeline?type=say&ajax=1&page={page}",
            self.uid.to_string()
        ))
    }
}

async fn fetch_user_info(uid: Uid) -> anyhow::Result<InitUser> {
    let url = Compass::new(uid).home();
    let fetcher = fetcher::get_bangumi();
    let html = fetcher.get(url).send().await?.text().await?;
    parser::user::parse_userpage(&html)
}

async fn fetch_names_update_until_key_point(
    uid: Uid,
    key_point: &str,
) -> anyhow::Result<Option<NamesUpdate>> {
    let mut page = 1;
    let mut all_names = std::collections::HashSet::new();
    let mut kp = None;
    let mut compass = Compass::new(uid.clone());
    loop {
        let url = compass.timeline_say_with_page(page);
        tracing::debug!("Fetching timeline page {}: {}", page, url);
        let fetcher = fetcher::get_bangumi();
        let ret = fetcher.get(&url).send().await?;
        let final_url = ret.url().to_string();
        tracing::debug!("Final URL: {}", final_url);
        if !final_url.eq(&url) {
            tracing::warn!("Redirected to {}, try fetching with new UID", final_url);
            let sid = final_url.split('/').nth(4).unwrap_or("");
            compass.uid = Uid::from_str(sid);
            continue;
        }
        if !ret.status().is_success() {
            tracing::error!("Failed to fetch timeline page {}: {}", page, ret.status());
            return Err(anyhow::anyhow!(
                "Failed to fetch timeline page {}: {}",
                page,
                ret.status()
            ));
        }
        let html = ret.text().await?;
        let name_history = parser::user::parse_timeline_name_history(&html);
        let Ok(name_history) = name_history else {
            tracing::error!("Failed to parse timeline page: {name_history:?}");
            return Err(anyhow::anyhow!(
                "Failed to parse timeline page: {name_history:?}"
            ));
        };
        tracing::debug!("Parsed name history: {:?}", name_history);
        if let Some(name_history) = name_history {
            if kp.is_none() {
                kp = Some(name_history.key_point.clone());
            }
            all_names.extend(name_history.names);
            if name_history.key_point == key_point {
                break;
            }
            page += 1;
        } else {
            break;
        }
    }
    let names_update = kp.map(|key_point| NamesUpdate {
        key_point,
        names: all_names,
    });
    Ok(names_update)
}

async fn init_user_data(uid: Uid) -> anyhow::Result<User> {
    let user = fetch_user_info(uid.clone()).await?;
    tracing::debug!("Fetched user info: {:?}", user);
    let uid = if let Some(sid) = user.sid.clone() {
        Uid::Sid(sid)
    } else {
        uid
    };
    let existing_user = service::user::find_by_uid(uid.clone()).await?;
    if let Some(existing_user) = existing_user {
        tracing::debug!("User already exists: {:?}", existing_user);
        return Ok(existing_user);
    }
    tracing::debug!("Fetched user info: {:?}", user);
    service::user::init_user(user).await
}

pub async fn query_user(uid: Uid) -> anyhow::Result<User> {
    let user = service::user::find_by_uid(uid.clone()).await?;
    let user = if let Some(user) = user {
        user
    } else {
        init_user_data(uid.clone()).await?
    };
    let result = user.clone();
    let queue = &QUEUE;
    let key = uid.clone();
    let task = || async move {
        if let Some(name_history) = &user.extra.name_history {
            let is_fresh = name_history.update_at > chrono::Utc::now() - chrono::Duration::days(1);
            if is_fresh || user.state != model::common::user::UserState::Active {
                return Ok(user);
            }
        };
        let sid = user.sid.clone();
        let nid = user.nid.clone();
        let uid = sid.map_or_else(|| Uid::Nid(nid.unwrap()), |sid| Uid::Sid(sid));
        let names_update = fetch_names_update_until_key_point(uid.clone(), "").await;
        let Ok(names_update) = names_update else {
            tracing::error!("Failed to fetch names update: {:?}", names_update);
            return Ok(user);
        };
        tracing::debug!("Fetched names update: {:?}", names_update);
        let Some(names_update) = names_update else {
            tracing::debug!("No names update found: {:?}", names_update);
            return Ok(user);
        };
        service::user::update_name_history(uid, names_update).await
    };

    tokio::spawn(async move {
        let _user = queue.get_or_spawn(key, task).await;
    });
    Ok(result)
}

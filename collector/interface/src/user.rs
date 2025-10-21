use crate::common::TaskQueue;
use std::sync::LazyLock;

use anyhow::anyhow;
use chrono::Utc;
use model::common::user::{InitUser, NamesUpdate, Uid, UserState};
use model::prelude::User;

static NAME_QUEUE: LazyLock<TaskQueue<Uid, User>> = LazyLock::new(|| TaskQueue::new(10));
static HOME_QUEUE: LazyLock<TaskQueue<Uid, User>> = LazyLock::new(|| TaskQueue::new(10));

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
    let url = Compass::new(uid.clone()).home();
    let fetcher = fetcher::get_bangumi();
    let html = fetcher.get(url).send().await?.text().await?;
    let mut init = InitUser::default();
    init.update_uid(uid);
    parser::user::parse_userpage(&html, Some(init))
}

async fn fetch_names_update_until_key_point(
    uid: Uid,
    key_point: chrono::DateTime<chrono::Utc>,
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
        let html = ret.text().await.expect("Fetch HTML Error");
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
            if key_point > name_history.key_point {
                break;
            }
            page += 1;
        } else {
            if kp.is_none() {
                kp = Some(Utc::now());
            }
            break;
        }
    }
    let names_update = kp.map(|key_point| NamesUpdate {
        key_point,
        names: all_names,
    });
    Ok(names_update)
}

async fn update_user_data(uid: Uid) -> anyhow::Result<User> {
    let queue = &HOME_QUEUE;
    let key = uid.clone();
    let task = async move || {
        let user = fetch_user_info(uid.clone()).await?;
        tracing::debug!("Fetched user info: {:?}", user);
        service::user::upsert_user(user).await
    };
    queue
        .get_or_spawn(key, task)
        .await
        .map_err(|err| anyhow!("Failed to update user data: {:?}", err))
}

async fn update_name_history(uid: Uid, user: User) -> anyhow::Result<User> {
    let key = uid.clone();
    let queue = &NAME_QUEUE;
    let task = || async move {
        let key_point = if let Some(name_history) = &user.extra.name_history {
            if !is_expired(name_history.update_at, &user.state) {
                return Ok(user);
            }
            name_history.key_point
        } else {
            chrono::DateTime::<chrono::Utc>::MIN_UTC
        };

        let sid = user.sid.clone();
        let nid = user.nid.clone();
        let uid = sid.map_or_else(|| Uid::Nid(nid.unwrap()), |sid| Uid::Sid(sid));
        let names_update = fetch_names_update_until_key_point(uid.clone(), key_point).await;
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
    queue
        .get_or_spawn(key, task)
        .await
        .map_err(|err| anyhow!("Failed to update user data: {:?}", err))
}

async fn update_user_data_if_expired(uid: Uid, user: User) -> anyhow::Result<User> {
    let user = if is_expired(user.update_at, &user.state) {
        update_user_data(uid.clone()).await?
    } else {
        user
    };
    if let Some(name_history) = &user.extra.name_history {
        if !is_expired(name_history.update_at, &user.state) {
            return Ok(user);
        }
    }
    update_name_history(uid, user).await
}

fn get_fresh_duration(state: &UserState) -> chrono::Duration {
    let dur = match state {
        UserState::Active => config::get().collector.user.fresh_duration.active,
        UserState::Abondon => config::get().collector.user.fresh_duration.abondon,
        UserState::Dropped => config::get().collector.user.fresh_duration.dropped,
        UserState::Banned => config::get().collector.user.fresh_duration.banned,
    };
    chrono::Duration::days(dur)
}

fn is_expired(update_at: chrono::DateTime<chrono::Utc>, state: &UserState) -> bool {
    update_at < chrono::Utc::now() - get_fresh_duration(state)
}

pub async fn query_user(uid: Uid) -> anyhow::Result<User> {
    let user = service::user::find_by_uid(uid.clone()).await?;
    let user = if let Some(user) = user {
        user
    } else {
        update_user_data(uid.clone()).await?
    };
    let result = user.clone();
    tokio::spawn(async move { update_user_data_if_expired(uid, user).await });
    Ok(result)
}

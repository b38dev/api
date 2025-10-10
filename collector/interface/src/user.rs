use model::{
    common::user::{InitUser, NamesUpdate, Uid},
    prelude::{Names, User},
};

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

pub async fn fetch_user_info(uid: Uid) -> anyhow::Result<InitUser> {
    let url = Compass::new(uid).home();
    let fetcher = fetcher::get_bangumi();
    let html = fetcher.get(url).send().await?.text().await?;
    parser::user::parse_userpage(&html)
}

pub async fn fetch_names_update_until_key_point(
    uid: Uid,
    key_point: &str,
) -> anyhow::Result<NamesUpdate> {
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
        let name_history = parser::user::parse_timeline_name_history(&html)?;
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
    Ok(NamesUpdate {
        key_point: kp.unwrap_or_else(|| key_point.to_string()),
        names: all_names,
    })
}

pub async fn init_user_data(uid: Uid) -> anyhow::Result<User> {
    let mut user = fetch_user_info(uid.clone()).await?;
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
    let names_update = fetch_names_update_until_key_point(uid.clone(), "").await?;
    tracing::debug!("Fetched user info: {:?}", user);
    tracing::debug!("Fetched names update: {:?}", names_update);
    user.names_update = Some(names_update);
    service::user::init_user(user).await
}

pub async fn refresh_name_history(uid: Uid) -> anyhow::Result<Names> {
    let user = service::user::find_by_uid(uid.clone()).await?;
    if user.is_none() {
        let user = init_user_data(uid).await?;
        return Ok(user.extra.into());
    }
    let user = user.unwrap();
    let is_fresh = user.update_at > chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
    if is_fresh || user.state != model::common::user::UserState::Active {
        return Ok(user.extra.into());
    }
    let uid = user
        .sid
        .map_or_else(|| Uid::Nid(user.nid.unwrap()), |sid| Uid::Sid(sid));
    let names_update = fetch_names_update_until_key_point(uid.clone(), "").await?;
    service::user::update_name_history(uid, names_update).await
}

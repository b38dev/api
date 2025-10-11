use sea_orm::prelude::DateTimeUtc;
use sea_orm::{DeriveValueType, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use std::{
    collections::{self, HashSet},
    str::FromStr,
};

pub use crate::entity::user::{Nid, Sid};

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub enum Uid {
    Nid(Nid),
    Sid(Sid),
}

impl Uid {
    pub fn from_str(s: &str) -> Self {
        if let Ok(nid) = s.parse() {
            Self::Nid(nid)
        } else {
            Self::Sid(s.to_string())
        }
    }

    pub fn is_nid(&self) -> bool {
        matches!(self, Uid::Nid(_))
    }

    pub fn is_sid(&self) -> bool {
        matches!(self, Uid::Sid(_))
    }
}

impl ToString for Uid {
    fn to_string(&self) -> String {
        match self {
            Uid::Nid(nid) => nid.to_string(),
            Uid::Sid(sid) => sid.clone(),
        }
    }
}

pub type Names = HashSet<String>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NameHistory {
    pub update_at: DateTimeUtc,
    pub key_point: DateTimeUtc,
    pub names: Names,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SubjectState {
    Do,      // 在X
    Collect, // X过
    Wish,    // 想X
    #[serde(rename = "on_hold")]
    OnHold, // 搁置
    Dropped, // 抛弃
}

impl FromStr for SubjectState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "do" => Ok(SubjectState::Do),
            "collect" => Ok(SubjectState::Collect),
            "wish" => Ok(SubjectState::Wish),
            "on_hold" => Ok(SubjectState::OnHold),
            "dropped" => Ok(SubjectState::Dropped),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SubjectType {
    Anime,
    Game,
    Book,
    Music,
    Real,
}

impl FromStr for SubjectType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "anime" => Ok(SubjectType::Anime),
            "game" => Ok(SubjectType::Game),
            "book" => Ok(SubjectType::Book),
            "music" => Ok(SubjectType::Music),
            "real" => Ok(SubjectType::Real),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypedCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doing: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wish: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped: Option<usize>,
}

impl TypedCollection {
    pub fn build(list: Vec<(String, usize)>) -> Option<Self> {
        if list.len() < 1 {
            return None;
        }
        let mut result = Self {
            doing: None,
            collect: None,
            wish: None,
            on_hold: None,
            dropped: None,
        };
        let mut s = false;
        for (state, count) in list {
            if let Ok(state) = state.parse() {
                s = true;
                let count = Some(count);
                match state {
                    SubjectState::Do => result.doing = count,
                    SubjectState::Collect => result.collect = count,
                    SubjectState::Wish => result.wish = count,
                    SubjectState::OnHold => result.on_hold = count,
                    SubjectState::Dropped => result.dropped = count,
                }
            };
        }
        if s { Some(result) } else { None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Collections {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anime: Option<TypedCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<TypedCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<TypedCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<TypedCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real: Option<TypedCollection>,
}

impl Collections {
    pub fn build(list: Vec<(SubjectType, Option<TypedCollection>)>) -> Option<Self> {
        if list.len() < 1 {
            return None;
        }
        let mut result = Self {
            anime: None,
            game: None,
            book: None,
            music: None,
            real: None,
        };
        let mut s = false;
        for (st, collection) in list {
            if collection.is_none() {
                continue;
            }
            s = true;
            match st {
                SubjectType::Anime => result.anime = collection,
                SubjectType::Game => result.game = collection,
                SubjectType::Book => result.book = collection,
                SubjectType::Music => result.music = collection,
                SubjectType::Real => result.real = collection,
            }
        }
        if s { Some(result) } else { None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, DeriveValueType)]
#[sea_orm(value_type = "String")]
#[serde(rename_all = "lowercase")]
pub enum UserState {
    Active,
    Abondon,
    Dropped,
    Banned,
}

impl ToString for UserState {
    fn to_string(&self) -> String {
        match self {
            UserState::Active => "active".to_string(),
            UserState::Abondon => "abondon".to_string(),
            UserState::Dropped => "dropped".to_string(),
            UserState::Banned => "banned".to_string(),
        }
    }
}

impl FromStr for UserState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(UserState::Active),
            "abondon" => Ok(UserState::Abondon),
            "dropped" => Ok(UserState::Dropped),
            "banned" => Ok(UserState::Banned),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult)]
pub struct Extra {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_history: Option<NameHistory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Collections>,
}

impl Default for Extra {
    fn default() -> Self {
        Self {
            name_history: None,
            collections: None,
        }
    }
}

impl Extra {
    pub fn merge(&mut self, other: Extra) -> &mut Self {
        if let Some(nh) = other.name_history {
            self.name_history = Some(nh);
        }
        if let Some(c) = other.collections {
            self.collections = Some(c);
        }
        self
    }

    pub fn update_collections(&mut self, collections: Collections) -> &mut Self {
        self.collections = Some(collections);
        self
    }

    pub fn update_collections_opt(&mut self, collections: Option<Collections>) -> &mut Self {
        if let Some(c) = collections {
            self.collections = Some(c);
        }
        self
    }

    pub fn replace_name_history(&mut self, name_history: NameHistory) -> &mut Self {
        self.name_history = Some(name_history);
        self
    }

    pub fn update_name_history(&mut self, name_history: NamesUpdate) -> &mut Self {
        if let Some(nh) = &mut self.name_history {
            nh.update_at = chrono::Utc::now();
            nh.key_point = name_history.key_point;
            nh.names.extend(name_history.names);
        } else {
            self.name_history = Some(NameHistory {
                update_at: chrono::Utc::now(),
                key_point: name_history.key_point,
                names: name_history.names,
            });
        }
        self
    }

    pub fn update_name_history_opt(&mut self, name_history: Option<NamesUpdate>) -> &mut Self {
        if let Some(nh) = name_history {
            self.update_name_history(nh);
        }
        self
    }
}

#[derive(Debug, Clone)]
pub struct InitUser {
    pub nid: Option<Nid>,
    pub sid: Option<Sid>,
    pub name: String,
    pub join_time: Option<DateTimeUtc>,
    pub last_active: Option<DateTimeUtc>,
    pub state: UserState,
    pub collections: Option<Collections>,
    pub names_update: Option<NamesUpdate>,
}

impl Default for InitUser {
    fn default() -> Self {
        Self {
            nid: None,
            sid: None,
            name: String::new(),
            join_time: None,
            last_active: None,
            state: UserState::Abondon,
            collections: None,
            names_update: None,
        }
    }
}

impl InitUser {
    pub fn update_uid(&mut self, uid: Uid) -> &mut Self {
        match uid {
            Uid::Nid(nid) => self.nid = Some(nid),
            Uid::Sid(sid) => self.sid = Some(sid),
        }
        self
    }

    pub fn update_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn update_state(&mut self, state: UserState) -> &mut Self {
        self.state = state;
        self
    }

    pub fn update_join_time(&mut self, join_time: DateTimeUtc) -> &mut Self {
        self.join_time = Some(join_time);
        self
    }

    pub fn update_join_time_opt(&mut self, join_time: Option<DateTimeUtc>) -> &mut Self {
        if let Some(jt) = join_time {
            self.join_time = Some(jt);
        }
        self
    }

    pub fn update_last_active(&mut self, last_active: DateTimeUtc) -> &mut Self {
        self.last_active = Some(last_active);
        self
    }

    pub fn update_last_active_opt(&mut self, last_active: Option<DateTimeUtc>) -> &mut Self {
        if let Some(la) = last_active {
            self.last_active = Some(la);
        }
        self
    }

    pub fn update_collections(&mut self, collections: Collections) -> &mut Self {
        self.collections = Some(collections);
        self
    }

    pub fn update_collections_opt(&mut self, collections: Option<Collections>) -> &mut Self {
        if let Some(c) = collections {
            self.collections = Some(c);
        }
        self
    }

    pub fn update_names_update(&mut self, names_update: NamesUpdate) -> &mut Self {
        self.names_update = Some(names_update);
        self
    }

    pub fn update_names_update_opt(&mut self, names_update: Option<NamesUpdate>) -> &mut Self {
        if let Some(nu) = names_update {
            self.names_update = Some(nu);
        }
        self
    }

    pub fn set_nid(&mut self, nid: Option<Nid>) -> &mut Self {
        self.nid = nid;
        self
    }

    pub fn set_sid(&mut self, sid: Option<Sid>) -> &mut Self {
        self.sid = sid;
        self
    }

    pub fn set_join_time(&mut self, join_time: Option<DateTimeUtc>) -> &mut Self {
        self.join_time = join_time;
        self
    }

    pub fn set_last_active(&mut self, last_active: Option<DateTimeUtc>) -> &mut Self {
        self.last_active = last_active;
        self
    }

    pub fn set_names_update(&mut self, names_update: Option<NamesUpdate>) -> &mut Self {
        self.names_update = names_update;
        self
    }

    pub fn set_collections(&mut self, collections: Option<Collections>) -> &mut Self {
        self.collections = collections;
        self
    }
}

#[derive(Debug, Clone)]
pub struct NamesUpdate {
    pub key_point: DateTimeUtc,
    pub names: Names,
}

impl From<Extra> for Names {
    fn from(extra: Extra) -> Self {
        let Some(nh) = extra.name_history else {
            return collections::HashSet::new();
        };
        nh.names
    }
}

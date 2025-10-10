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
    pub update_at: chrono::DateTime<chrono::Utc>,
    pub key_point: String,
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

#[derive(Debug, Clone)]
pub struct InitUser {
    pub nid: Option<Nid>,
    pub sid: Option<Sid>,
    pub name: String,
    pub join_time: Option<chrono::DateTime<chrono::Utc>>,
    pub state: UserState,
    pub collections: Option<Collections>,
    pub names_update: Option<NamesUpdate>,
}

#[derive(Debug, Clone)]
pub struct NamesUpdate {
    pub key_point: String,
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

use crate::common::user::{Extra, UserState};
use sea_orm::entity::prelude::*;

pub type Nid = i32;
pub type Sid = String;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub nid: Option<Nid>,
    #[sea_orm(unique)]
    pub sid: Option<Sid>,
    pub name: String,
    pub state: UserState,
    pub join_time: Option<DateTime>,
    pub update_at: DateTime,
    #[sea_orm(column_type = "JsonBinary")]
    pub extra: Extra,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

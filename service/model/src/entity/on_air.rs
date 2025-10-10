use sea_orm::entity::prelude::*;

pub type SubjectId = i32;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "on_air")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub subject: SubjectId,
    #[sea_orm(column_type = "JsonBinary")]
    pub data: crate::common::onair::BangumiItem,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(integer_null(User::Nid).unique_key())
                    .col(string_null(User::Sid).unique_key())
                    .col(string_null(User::Name))
                    .col(string(User::State))
                    .col(timestamp_null(User::JoinTime))
                    .col(timestamp(User::UpdateAt))
                    .col(json_binary(User::Extra))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Nid,
    Sid,
    Name,
    State,
    JoinTime,
    UpdateAt,
    Extra,
}

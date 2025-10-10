use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KeyValue::Table)
                    .if_not_exists()
                    .col(string(KeyValue::Key).primary_key())
                    .col(json_binary(KeyValue::Value))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KeyValue::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum KeyValue {
    Table,
    Key,
    Value,
}

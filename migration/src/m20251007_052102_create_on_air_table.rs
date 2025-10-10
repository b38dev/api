use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OnAir::Table)
                    .if_not_exists()
                    .col(integer(OnAir::Subject).primary_key())
                    .col(json_binary(OnAir::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OnAir::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OnAir {
    Table,
    Subject,
    Data,
}

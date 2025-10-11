pub use sea_orm_migration::prelude::*;

mod m20251007_052102_create_on_air_table;
mod m20251007_052102_create_user_table;
mod m20251008_034436_create_key_value_table;
mod m20251011_062841_alter_user_timestamp;
mod m20251011_112458_alter_user_timestamp;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251007_052102_create_on_air_table::Migration),
            Box::new(m20251007_052102_create_user_table::Migration),
            Box::new(m20251008_034436_create_key_value_table::Migration),
            Box::new(m20251011_062841_alter_user_timestamp::Migration),
            Box::new(m20251011_112458_alter_user_timestamp::Migration),
        ]
    }
}

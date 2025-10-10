pub use sea_orm::*;
use tokio::sync::OnceCell;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub type Db = DatabaseConnection;

pub async fn init_db() -> anyhow::Result<DatabaseConnection> {
    let db = sea_orm::Database::connect(config::get().database.get_uri()).await?;
    DB.set(db.clone())?;
    Ok(db)
}

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().expect("Database not initialized")
}

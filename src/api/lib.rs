use std::sync::Arc;
use sea_orm::DatabaseConnection;
use anyhow::Result;

use super::service::{StockServiceTrait, StockService};

pub struct AppContext {
    /// The database connections
    pub db: Arc<DatabaseConnection>,

    pub stocks: Arc<dyn StockServiceTrait>,
}

impl AppContext {
	/// Create a new set of dependencies based on the given shared resources
    pub async fn init(datbase_url: &str) -> Result<Self> {
        let db = Arc::new(sea_orm::Database::connect(datbase_url).await?);

        Ok(Self {
			stocks: Arc::new(StockService::new(&db)),
            db
        })
    }
}
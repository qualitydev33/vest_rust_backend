use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};
use entity::stock::{Model as Stock, self};

pub struct StockService {
	db: Arc<DatabaseConnection>,
}

impl StockService {
	pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
pub trait StockServiceTrait: Sync + Send {
	async fn get_stock_by_symbol(&self, symbol: &str) -> Result<Option<Stock>>;
	async fn buy_stock(&self) -> String;
	async fn sell_stock(&self) -> String;
}



#[async_trait]
impl StockServiceTrait for StockService {
	async fn get_stock_by_symbol(&self, symbol: &str) -> Result<Option<Stock>> {
		println!("symbol is {}", symbol);
		let result = stock::Entity::find_by_id(1).one(&*self.db).await?;
		Ok(result)
	}
	async fn buy_stock(&self) -> String {
		"buy_stock service called".to_string()
	}
	async fn sell_stock(&self) -> String {
		"sell_stock service called".to_string()
	}
}


pub struct StockLoader {
	locations: Arc<dyn StockServiceTrait> 
}

impl StockLoader {
	pub fn new(locations: &Arc<dyn StockServiceTrait>) -> Self {
		Self { locations: locations.clone() }
	}
}

// #[async_trait]
// impl Loader<String> for StockLoader {
//     type Value = Stock;
//     type Error = FieldError;

//     async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
//         let profiles = self.stocks.get_by_ids(keys.into()).await?;
		
//         Ok(profiles
//             .into_iter()
//             .map(|profile| (profile.id.clone(), profile))
//             .collect());
//     }
// }
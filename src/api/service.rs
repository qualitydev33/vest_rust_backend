use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use entity::{
	stock_entity::{Model as StockEntity, self}, 
	stock_order_entity::{Model as StockOrderEntity, self, Entity as StockOrder}
};

use crate::{kafka::producer::use_producer, utils::errors::AppError, external_api::nasdaq::get_nasdaq_stock_by_symbol};

use super::resolver::StockOrderInput;

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
	async fn get_stock_by_symbol(&self, symbol: &str) -> Result<Option<StockEntity>>;
	async fn get_stock_order_list(&self) -> Result<Vec<StockOrderEntity>>;
	async fn create_order(&self, payload: StockOrderInput, order_type: &str) -> Result<StockOrderEntity>;
}



#[async_trait]
impl StockServiceTrait for StockService {
	async fn get_stock_by_symbol(&self, symbol: &str) -> Result<Option<StockEntity>> {
		// let result = stock_entity::Entity::find()
		// 	.filter(stock_entity::Column::Symbol.eq(symbol))
		// 	.one(&*self.db)
		// 	.await
		// 	.unwrap();
		let result = get_nasdaq_stock_by_symbol(symbol).await;
		Ok(result)
		
	}
	async fn get_stock_order_list(&self) -> Result<Vec<StockOrderEntity>> {
		let result = stock_order_entity::Entity::find()
			.all(&*self.db)
			.await?;
		Ok(result)
	}
	async fn create_order(&self, order_input: StockOrderInput, order_type: &str) -> Result<StockOrderEntity, anyhow::Error> {
		let order_model = StockOrderEntity::new(order_input.symbol, order_input.bid_price, order_input.bid_size, order_type.to_string());
		
		// send message through kafka
		let raw_data = serde_json::to_string(&order_model).unwrap();
		let topic = std::env::var("KAFKA_ORDER_TOPIC").expect("You've not set the kafka topic");
		let delivery_result = use_producer(&topic, raw_data).await;
		
		match delivery_result {
			Ok(_) => {
				Ok(order_model)
			},
			Err(_) => {
				panic!("Kafka error occured!")			
			}
		}
	}
}
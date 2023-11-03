use async_graphql::{ Object, Context, Result, InputObject};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::{api::service::StockServiceTrait, utils::errors::AppError};
use entity::{
	stock_entity::StockEntity,
	stock_order_entity::StockOrderEntity
};

#[derive(Default)]
pub struct  StockQuery {}

#[derive(Default)]
pub struct StockMutation {}

#[Object]
impl StockQuery {
	async fn get_stock_by_symbol(
		&self,
		ctx: &Context<'_>,
		#[graphql(desc = "The Stock Symbol")] symbol: String,
	) -> Result<Option<StockEntity>, AppError> {
		let stock_service = ctx.data_unchecked::<Arc<dyn StockServiceTrait>>();
		let stock = stock_service.get_stock_by_symbol(&symbol).await?;
		Ok(stock)
	}
	async fn get_stock_order_list(&self, ctx: &Context<'_>) -> Result<Vec<StockOrderEntity>, AppError> {
		let stock_service = ctx.data_unchecked::<Arc<dyn StockServiceTrait>>();
		let orders = stock_service.get_stock_order_list().await?;
		Ok(orders)
	}
}

#[Object]
impl StockMutation {
	async fn buy_stock(
		&self,
		ctx: &Context<'_>,
		order_input: StockOrderInput
	) -> Result<StockOrderEntity, AppError> {
		let stock_service = ctx.data_unchecked::<Arc<dyn StockServiceTrait>>();
		let result = stock_service.create_order(order_input, "buy").await?;
		Ok(result)
	
	}
	async fn sell_stock(
		&self,
		ctx: &Context<'_>,
		order_input: StockOrderInput
	) -> Result<StockOrderEntity, AppError> {
		let stock_service = ctx.data_unchecked::<Arc<dyn StockServiceTrait>>();
		let result = stock_service.create_order(order_input, "sell").await?;
		Ok(result)
	}
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct StockOrderInput {
	pub symbol: String,
	pub bid_price: f32,
	pub bid_size: i32,
}
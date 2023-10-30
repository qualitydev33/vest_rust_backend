use async_graphql::{ Object, Context};

#[derive(Default)]
pub struct  StockQuery {}

#[derive(Default)]
pub struct StockMutation {}

#[Object]
impl StockQuery {
	async fn get_stock_by_symbol(&self, ctx: &Context<'_>) -> String {
		println!("get_stock_by_symbol called!");
		// StockService::get_stock_by_symbol(self, "symbol");
		// StockService::new()
		"get_stock_by_symbol called!".to_string()
	}
	async fn get_stock_list_held(&self, ctx: &Context<'_>) -> String {
		println!("get_stock_list_held called!");
		"get_stock_list_held called!".to_string()
	}
}

#[Object]
impl StockMutation {
	async fn buy_stock(
		&self,
		ctx: &Context<'_>,
		input: String
	) -> String {
		println!("buy_stock mutation called!");
		"buy_stock mutation called!".to_string()	
	}
	async fn sell_stock(&self) -> String {
		println!("sell_stock mutation called!");
		"sell_stock mutation called!".to_string()
	}
}
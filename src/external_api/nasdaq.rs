use entity::stock_entity::StockEntity;
use reqwest::{self, Client};


pub async fn get_nasdaq_stock_by_symbol(symbol: &str) -> Option<StockEntity> {
	let client = Client::new();
	let resp = client.get(&format!("https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks", symbol)).send().await.unwrap();
	println!("url=={}", resp.url().as_str().to_string());
	if resp.status().is_success() {
		let body = resp.text().await.unwrap();
		let stock: StockEntity = serde_json::from_str(&body).unwrap();
		Some(stock)
	} else {
		None
	}
}
use anyhow::Result;
use sea_orm::DatabaseConnection;

pub async fn buy_stock(
	db_connection: DatabaseConnection,
	payload: Stock,
	amount: i32,
) {

}
struct StocksQuery;

impl StocksQuery {
    async fn find_stock_by_symbol(&self) -> &'static str {
        "findStockBySymbol Query"
    }
    async fn buy_stock(&self) -> &'static str {
        "buyStock Query"
    }
    async fn sell_stock(&self) -> &'static str {
        "getStock Query"
    }
}
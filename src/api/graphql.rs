use anyhow::Result;
use async_graphql::{dataloader::DataLoader, EmptySubscription, MergedObject, Schema};
use std::sync::Arc;
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::GraphiQLSource;

use super::{
	resolver::{StockQuery, StockMutation},
	service::StockLoader, lib::AppContext
};


#[derive(MergedObject, Default)]
pub struct Query(
	StockQuery
);

#[derive(MergedObject, Default)]
pub struct Mutation(
	StockMutation
);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn graphql_handler(
    Extension(schema): Extension<GraphQLSchema>,
    Extension(ctx): Extension<Arc<AppContext>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    // Add the Subject and optional User to the context
    // let request = req.into_inner().data(sub).data(user);

    schema.execute(req.into_inner()).await.into()
}

/// Initialize all necessary dependencies to create a `GraphQLSchema`. Very simple dependency
/// injection based on async-graphql's `.data()` calls.
pub fn create_schema(ctx: Arc<AppContext>) -> Result<GraphQLSchema> {
    // Instantiate loaders
    let stock_loader = StockLoader::new(&ctx.stocks);

    // Inject the initialized services into the `Schema` instance.
    Ok(
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
			.data(ctx.stocks.clone())
            .data(DataLoader::new(stock_loader, tokio::spawn))
            .finish(),
    )
}

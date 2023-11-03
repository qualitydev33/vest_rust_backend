use anyhow::Result;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use std::sync::Arc;
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::GraphiQLSource;

use super::{
	resolver::{StockQuery, StockMutation},
	lib::AppContext
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
    schema.execute(req.into_inner()).await.into()
}

pub fn create_schema(ctx: Arc<AppContext>) -> Result<GraphQLSchema> {
    Ok(
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
			.data(ctx.stocks.clone())
            .finish(),
    )
}

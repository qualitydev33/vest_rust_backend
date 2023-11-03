use std::{
    error::Error,
    sync::Arc, thread, time::Duration
};
use anyhow::Result;
use dotenv::dotenv;
use axum::{Router, Server, routing::get, extract::Extension};
use rdkafka::{producer::{BaseProducer, BaseRecord, FutureProducer, FutureRecord}, ClientConfig, util::Timeout};
use tower_http::trace::{self, TraceLayer};

mod api;
mod kafka;
mod utils;
mod external_api;

use api::lib::AppContext;
use tracing::info;
use crate::{
    api::graphql::{create_schema, graphiql, graphql_handler},
    kafka::{
        consumer::use_consumer, 
        producer::use_producer
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let port = std::env::var("PORT").expect("You've not set the port.");
    let database_url = std::env::var("DATABASE_URL").expect("You've not set the Database url.");

    let ctx = Arc::new(AppContext::init(&database_url).await?);
    let schema = create_schema(ctx.clone())?;

    let topic = std::env::var("KAFKA_ORDER_TOPIC").expect("You've not set the kafka topic");
    let _ = use_consumer(&topic);
    
    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(Extension(ctx))
        .layer(Extension(schema));
        

    println!("GraphiQL Server running on http://localhost:{}/graphql", port);
    
    Server::bind(
        &format!("0.0.0.0:{}", port)
        .parse().expect("Unable to parse bind address"))
        .serve(app.into_make_service()).await?;
    
    Ok(())
}
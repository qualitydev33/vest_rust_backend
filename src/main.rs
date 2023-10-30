use std::{
    error::Error,
    sync::Arc,
    thread, time::Duration
};
use anyhow::Result;
use dotenv::dotenv;
use axum::{Router, Server, routing::get, extract::Extension};
use tower_http::trace::{self, TraceLayer};
use rdkafka::{
    ClientConfig,
    consumer::{BaseConsumer, Consumer, StreamConsumer},
    producer::{BaseProducer, BaseRecord}
};

mod api;
mod kafka;
use api::lib::AppContext;
use crate::api::graphql::{create_schema, graphiql, graphql_handler};
use kafka::{
    producer::{get_producer, use_producer},
    consumer::{get_consumer, use_consumer}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let port = std::env::var("PORT").expect("You've not set the port.");
    let kafka_port = std::env::var("KAFKA_PORT").expect("You've not set the kafka port");
    let database_url = std::env::var("DATABASE_URL").expect("You've not set the Database url.");

    

    // let producer: BaseProducer = ClientConfig::new()
    //     .set("bootstrap.servers", format!("localhost:{}", kafka_port))
    //     .create()
    //     .expect("Invalid producer config");

    // for i in 1..10 {
    //     println!("sending message");

    //     producer
    //         .send(
    //             BaseRecord::to("buy-sell-stock")
    //                 .key(&format!("key-{}", i))
    //                 .payload(&format!("value-{}", i)),
    //         )
    //         .expect("failed to send message");

    //     thread::sleep(Duration::from_secs(3));
    // }
    let producer = get_producer();
    // use_producer("buy-sell-stock", producer);

    // let consumer: StreamConsumer = ClientConfig::new()
    //     .set("bootstrap.servers", format!("localhost:{}", kafka_port))
    //     .set("group.id", "my_consumer_group")
    //     .create()?;
    // consumer
    //     .subscribe(&["buy-sell-stock"])
    //     .expect("topic subscribe failed");
    let consumer = get_consumer();
    use_consumer("buy-sell-stock", consumer);



    let ctx = Arc::new(AppContext::init(&database_url).await?);
    let schema = create_schema(ctx.clone())?;
    
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
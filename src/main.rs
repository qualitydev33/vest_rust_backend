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

    // let producer: BaseProducer = ClientConfig::new()
    //     .set("bootstrap.servers", "localhost:9092")
    //     .create()
    //     .expect("Invalid producer config");

    // for i in 1..5 {
    //     producer
    //         .send(BaseRecord::to(&topic)
    //             .key(&format!("key-{}", i))
    //             .payload(&format!("value-{}", i))
    //         )
    //         .expect("Failed to send message");
    //     thread::sleep(Duration::from_secs(2))
    // }

    // let producer: FutureProducer = ClientConfig::new()
    //     .set("bootstrap.servers", "localhost:9092")
    //     .create()
    //     .expect("Invalid producer config");
    
    // for i in 1..5 {
    //     let record = FutureRecord::to(&topic).payload("message payload").key("message key");
    //     let delivery_future = producer.send(record, Timeout::Never);
    //     delivery_future
    //         .map(|delivery_result| {
    //             // Handle delivery result here
    //             match delivery_result {
    //                 Ok(delivery_report) => {
    //                     // Message was successfully delivered to the broker
    //                     println!("Message delivered to topic: {}, partition: {}, offset: {}",
    //                              delivery_report.topic(),
    //                              delivery_report.partition(),
    //                              delivery_report.offset());
    //                 }
    //                 Err((kafka_error, message)) => {
    //                     // Error occurred during message delivery
    //                     println!("Failed to deliver message: {} - {}",
    //                              kafka_error,
    //                              message.payload_view::<str>().unwrap_or("Invalid UTF-8"));
    //                 }
    //             }
    //         })
    //         .wait()
    //         .unwrap();
    // }

    // let producer: &FutureProducer = &ClientConfig::new()
    //     .set("bootstrap.servers", "localhost:9092")
    //     .set("message.timeout.ms", "5000")
    //     .create()
    //     .expect("Producer creation error");

    // let futures = (0..5)
    //     .map(|i| async move {
    //         // The send operation on the topic returns a future, which will be
    //         // completed once the result or failure from Kafka is received.
    //         let delivery_status = producer
    //             .send(
    //                 FutureRecord::to(&topic_ref)
    //                     .payload(&format!("Message {}", i))
    //                     .key(&format!("Key {}", i)),
    //                 Duration::from_secs(0),
    //             )
    //             .await;

    //         // This will be executed when the result is received.
    //         info!("Delivery status for message {} received", i);
    //         delivery_status
    //     })
    //     .collect::<Vec<_>>();

    // // This loop will wait until all delivery statuses have been received.
    // for future in futures {
    //     println!("Future completed. Result: {:?}", future.await);
    // }
    
    // let delivery_result = use_producer(&topic, "payload".to_string()).await;
    // match delivery_result {
    //     Ok(value) => {println!("Delivered: {}", value.0)},
    //     Err(err) => {}
    // }
    
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
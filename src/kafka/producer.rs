use std::time::Duration;
use std::sync::Arc;
use rdkafka::{
    ClientConfig,
    producer::{FutureRecord, FutureProducer}, error::KafkaError, message::OwnedMessage
};
use anyhow::Result;

use crate::utils::errors::AppError;


pub async fn use_producer(
    topic: &str,
    payload:  String
) -> Result<(i32, i64), (KafkaError, OwnedMessage)> {
    let kafka_port = std::env::var("KAFKA_PORT").expect("You've not set the kafka port");
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", format!("localhost:{}", kafka_port))
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let delivery_status = producer
        .send(
            FutureRecord::to(topic)
                .payload(&payload)
                .key(&format!("Key")),
            Duration::from_secs(0),
        )
        .await;
    delivery_status
}
use std::{
    thread, time::Duration
};
use rdkafka::{
    ClientConfig,
    producer::{BaseProducer, BaseRecord, FutureRecord, FutureProducer}
};

pub fn get_producer() -> BaseProducer {
	let kafka_port = std::env::var("KAFKA_PORT").expect("You've not set the kafka port");
	let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", format!("localhost:{}", kafka_port))
        .create()
        .expect("Invalid producer config");
	producer
}

pub fn use_producer(
	topic: &str, 
	producer: FutureProducer
) {
    for i in 1..10 {
        println!("sending message");

        producer
            .send(
                FutureRecord::to(&topic)
                    .key(&format!("key-{}", i))
                    .payload(&format!("value-{}", i)),
					Duration::from_secs(1),
            );
    }
}
use rdkafka::{
    ClientConfig,
    consumer::{Consumer, StreamConsumer},
};

pub fn get_consumer() -> StreamConsumer {
	let kafka_port = std::env::var("KAFKA_PORT").expect("You've not set the kafka port");
	let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", format!("localhost:{}", kafka_port))
        .set("group.id", "my_consumer_group")
        .create()
		.expect("Unable to get consumer");
    consumer
}

pub fn use_consumer(topic: &str, consumer: StreamConsumer) {
	consumer
        .subscribe(&[topic])
        .expect("topic subscribe failed");
}
use entity::stock_order_entity::{StockOrderEntity, Entity as StockOrder};
use rdkafka::{
    ClientConfig,
    consumer::{Consumer, StreamConsumer, BaseConsumer}, Message, producer::{FutureProducer, FutureRecord}
};
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::warn;
use std::sync::Arc;

async fn connect_db_for_consumer() -> Result<Arc<DatabaseConnection>, sea_orm::DbErr> {
    let db_url = std::env::var("DATABASE_URL").expect("Failed to connect DB");
    let db = Arc::new(sea_orm::Database::connect(db_url).await?);
    Ok(db)
}

pub async fn create_consumer() -> StreamConsumer {
    let kafka_port = std::env::var("KAFKA_PORT").expect("You've not set the kafka port");
	let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", format!("localhost:{}", kafka_port))
        .set("group.id", "my_consumer_group")
        .create()
		.expect("Invalid Consumer Config");
    consumer
}

pub async fn use_consumer(topic: &str) {
	let consumer: StreamConsumer = create_consumer().await;
	consumer
        .subscribe(&[topic])
        .expect(&format!("{} topic subscribe failed", topic));

    // Consume messages from Kafka topic
    // let msg_stream = consumer.stream().next().await;
    
    loop {
        match consumer.recv().await {
            Err(e) => warn!("Error: {}", e),
            Ok(message) => {
                let payload = match message.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                let _ = std::env::var("KAFKA_ORDER_TOPIC").unwrap();
                if topic == std::env::var("KAFKA_ORDER_TOPIC").unwrap() {
                    let order: StockOrderEntity = serde_json::from_str(payload).unwrap();
                    let active_model = StockOrderEntity::into_active_model(&order);
                    let db = connect_db_for_consumer().await.expect("Unable to connect DB");
                    StockOrder::insert(active_model).exec(&*db).await.expect("Failed to insert");
                }  
            }
        }
    }
}


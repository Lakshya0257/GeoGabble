// // use kafka::{Error, Producer, ProducerBuilder};

// // // Kafka broker address and topic name
// // const BROKER_ADDRESS: &str = "localhost:9092";
// // const TOPIC_NAME: &str = "axum_logs";

// // async fn initialize_producer() -> Result<Producer, Error> {
// //     let producer = ProducerBuilder::new()
// //         .bootstrap_servers(BROKER_ADDRESS)
// //         .build()?;

// //     Ok(producer)
// // }

// // use rdkafka::{
// //     config::ClientConfig,
// //     consumer::stream_consumer::StreamConsumer,
// //     consumer::{Consumer, DefaultConsumerContext},
// //     message::BorrowedMessage,
// //     util::Timeout,
// // };

// use std::time::Duration;
// // use kafka::client::{Compression, KafkaClient, SecurityConfig};
// // use kafka::producer::{Producer, Record, RequiredAcks};
// // use kafka::error::Error as KafkaError;
// use rdkafka::config::ClientConfig;
// use rdkafka::producer::{BaseProducer, BaseRecord, FutureProducer, FutureRecord, Producer};

// pub async fn create_producer() {
//     println!("creating producer");

//     let producer: FutureProducer = ClientConfig::new()
//         .set("bootstrap.servers", "pkc-l7pr2.ap-south-1.aws.confluent.cloud:9092")
//         // .set("produce.offset.report", "true")
//         .set("message.timeout.ms", "60000")
//         .set("queue.buffering.max.messages", "10")
//         .set("security.protocol", "SASL_SSL")
//         .set("sasl.mechanisms", "PLAIN")
//         .set("sasl.username", "U43V2SY3A2ZOSRDJ")
//         .set("sasl.password", "WHA8FoZytE2gvqssPOwo7yOPijomJ/CUHq43BmgavGUBGc+5GOCUsRaCUPNsXtBa")
//         .create()
//         .expect("Producer creation error");

//     let future_result = producer.send(
//         FutureRecord::to("poems")
//             .payload("this is the payload")
//             .key("and this is a key"),
//         Duration::from_secs(0),
//     ).await;

//     match future_result {
//         Ok(_) => println!("Message sent successfully"),
//         Err((err, _)) => eprintln!("Failed to send message: {}", err),
//     }

//     // Poll and flush the producer
//     producer.poll(Duration::from_millis(100));
//     producer.flush(Duration::from_secs(1));
// }



// // async fn kafka_consumer() {
// //     // Configure Kafka consumer with your Confluent Cloud cluster credentials
// //     let consumer: StreamConsumer = ClientConfig::new()
// //         .set("bootstrap.servers", "pkc-l7pr2.ap-south-1.aws.confluent.cloud:9092")
// //         .set("sasl.mechanism", "PLAIN")
// //         .set("security.protocol", "SASL_SSL")
// //         .set("sasl.username", "U43V2SY3A2ZOSRDJ")
// //         .set("sasl.password", "WHA8FoZytE2gvqssPOwo7yOPijomJ/CUHq43BmgavGUBGc+5GOCUsRaCUPNsXtBa")
// //         .set("group.id", "my-group")
// //         .create()
// //         .expect("Consumer creation failed");

// //     // Subscribe to Kafka topics
// //     consumer
// //         .subscribe(&["my-topic"])
// //         .expect("Can't subscribe to specified topics");

// //     // Example loop to consume messages
// //     tokio::spawn(async move {
// //         loop {
// //             let result = consumer.recv().await;
// //             match result {
// //                 Ok(Ok(message)) => {
// //                     // Process the message
// //                     println!("Received message: {:?}", message);
// //                 }
// //                 Ok(Err(_)) => {
// //                     // Handle error
// //                 }
// //                 Err(_) => {
// //                     // Handle error
// //                 }
// //             }
// //         }
// //     });
// // }
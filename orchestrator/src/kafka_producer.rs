use rdkafka::producer::{FutureProducer, FutureRecord, DeliveryFuture};
use rdkafka::ClientConfig;

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> KafkaProducer {
        KafkaProducer {
            producer: ClientConfig::new()
                .set("bootstrap.servers", "localhost:9092")
                .create()
                .expect("Could not create Kafka Producer")
        }
    }

    pub fn start_request(&mut self, request: String) -> DeliveryFuture {
        let record: FutureRecord<String, String> = FutureRecord::to("simulation_requests")
            .payload(&request);
        self.producer.send(record, 0)
    }
}

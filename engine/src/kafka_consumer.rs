use std::error::Error;

use futures::StreamExt;
use rdkafka::ClientConfig;
use rdkafka::consumer::{DefaultConsumerContext, MessageStream, StreamConsumer};
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::message::Message;

use crate::config::Config;
use crate::epidemiology_simulation::Epidemiology;
use std::collections::HashMap;
use rand::Rng;

pub struct KafkaConsumer<'a> {
    engine_id: &'a str,
    consumer: StreamConsumer,
}

impl KafkaConsumer<'_> {
    pub fn new(engine_id: &str) -> KafkaConsumer {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("group.id", engine_id)
            .create()
            .expect("Consumer creation failed");

        let topics = ["simulation_requests"];
        consumer.subscribe(&topics)
            .expect("Can't subscribe to specified topics");

        return KafkaConsumer { engine_id, consumer };
    }

    pub async fn listen_loop(&self) {
        let mut message_stream: MessageStream<DefaultConsumerContext> = self.consumer.start();
        while let Some(message) = message_stream.next().await {
            let simulation_config = self.parse_message(message);
            match simulation_config {
                Err(e) => {
                    println!("Received a message, but could not parse it.\n\
                        Error Details: {}", e)
                }
                Ok(request) => {
                    self.run_sim(request)
                }
            };
        }
    }

    fn run_sim(&self, request: Request) {
        match request {
            Request::SimulationRequest(req) => {
                let mut epidemiology = Epidemiology::new(&req.config, req.sim_id);
                epidemiology.run(&req.config);
            }
            Request::MultiSimRequest(req) => {
                let sim_req = req.iter().find(|c| c.engine_id == self.engine_id);
                match sim_req {
                    None => { println!("Couldn't find any work for engine_id: {}", self.engine_id) }
                    Some(req) => {
                        let sim_id = req.config.sim_id.clone();
                        let mut epidemiology = Epidemiology::new(&req.config.config, sim_id);
                        epidemiology.run(&req.config.config);
                    }
                }
            }
        }
    }

    fn parse_message(&self, message: Result<BorrowedMessage, KafkaError>) -> Result<Request, Box<dyn Error>> {
        let borrowed_message = message?;
        let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
        serde_json::from_str(parsed_message).map_err(|e| e.into())
    }
}

#[derive(Debug, Deserialize)]
struct SimulationRequest {
    sim_id: String,
    #[serde(flatten)]
    config: Config,
}

#[derive(Debug, Deserialize)]
struct SimRequestByEngine {
    engine_id: String,
    config: SimulationRequest,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Request {
    SimulationRequest(SimulationRequest),
    MultiSimRequest(Vec<SimRequestByEngine>),
}

mod acceptor;
pub use acceptor::*;

mod publisher;
pub use publisher::*;

mod pubsub;
pub use pubsub::*;

mod router;
pub use router::*;

impl GetsubRequest {
    pub fn new(topic: &str) -> Self {
        Self {
            topic: topic.to_string(),
        }
    }
}

impl UnsubRequest {
    pub fn new(client: &str, topicfilters: Vec<String>) -> Self {
        Self {
            clientid: client.to_string(),
            topicfilters,
        }
    }
}

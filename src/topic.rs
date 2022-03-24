use crate::pb::{pub_sub_service_client::PubSubServiceClient, GetsubRequest, UnsubRequest};
use anyhow::Result;
use async_trait::async_trait;
use tonic::{transport::Channel, Request};

#[async_trait]
pub trait TopicService {
    async fn get_subs(&mut self, topic: &str) -> Result<Vec<String>>;
    async fn unsub(&mut self, client: &str, topicfilters: Vec<String>) -> Result<()>;
}

pub struct DefaultTopicService {
    subscriber_client: PubSubServiceClient<Channel>,
}

impl DefaultTopicService {
    pub fn new(subscriber_client: PubSubServiceClient<Channel>) -> Self {
        Self { subscriber_client }
    }
}

#[async_trait]
impl TopicService for DefaultTopicService {
    async fn get_subs(&mut self, topic: &str) -> Result<Vec<String>> {
        let resp = self
            .subscriber_client
            .get_subs(Request::new(GetsubRequest::new(topic)))
            .await?
            .into_inner();
        Ok(resp.subs)
    }

    async fn unsub(&mut self, client: &str, topicfilters: Vec<String>) -> Result<()> {
        self.subscriber_client
            .unsub(Request::new(UnsubRequest::new(client, topicfilters)))
            .await?;
        Ok(())
    }
}

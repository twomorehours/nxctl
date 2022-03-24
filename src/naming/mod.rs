use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

mod nacos;
pub use nacos::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Instance {
    ip: String,
    port: u32,
}

impl Instance {
    pub fn as_addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[async_trait]
pub trait Naming: Sync + Send {
    async fn get_instance_list(&self, service: &str) -> Result<Vec<Instance>>;
    async fn get_instance(&self, service: &str) -> Result<Option<Instance>> {
        let instances = self.get_instance_list(service).await?;
        let instance = if instances.is_empty() {
            None
        } else {
            Some(instances[0].clone())
        };
        Ok(instance)
    }
}

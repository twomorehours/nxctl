use super::{Instance, Naming};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct InstanceList {
    hosts: Vec<Instance>,
}

#[derive(Debug)]
pub struct NacosClient {
    addr: String,
    group: String,
    namespace: String,
}

impl NacosClient {
    pub fn new<T: Into<String>>(addr: T, group: T, namespace: T) -> Self {
        Self {
            addr: addr.into(),
            group: group.into(),
            namespace: namespace.into(),
        }
    }
}

#[async_trait]
impl Naming for NacosClient {
    async fn get_instance_list(&self, service: &str) -> Result<Vec<Instance>> {
        let url = format!(
            "http://{}/nacos/v1/ns/instance/list?serviceName={}&groupName={}&namespaceId={}&healthyOnly=true",
            self.addr,
            service,
            self.group,
            self.namespace,
        );
        let instance_list = reqwest::get(&url).await?.json::<InstanceList>().await?;
        Ok(instance_list.hosts)
    }
}

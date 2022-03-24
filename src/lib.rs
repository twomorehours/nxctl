use crate::topic::DefaultTopicService;
use anyhow::{anyhow, Result};
use clap::Parser;
use naming::{NacosClient, Naming};
use pb::pub_sub_service_client::PubSubServiceClient;
use tonic::transport::Channel;
use topic::TopicService;

mod pb;

mod naming;

mod topic;

#[derive(Parser, Debug)]
/// commandline tool for NXMQ
pub struct Args {
    /// 子命令
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    /// 查订阅
    GetSub {
        /// 主题名
        #[clap(short, long)]
        topic: String,
    },
    /// 删订阅
    Unsub {
        /// 主题名
        #[clap(short, long)]
        client: String,
        /// 主题filters
        #[clap(short, long)]
        topicfilters: Vec<String>,
    },
}

pub async fn do_action(args: Args) -> Result<()> {
    // parse env
    let nacos_addr =
        std::env::var("NXCTL_NACOS").unwrap_or_else(|_| "192.168.19.170:8848".to_string());

    // build client
    let naming = NacosClient::new(&nacos_addr[..], "MQTT_GATEWAY_GROUP", "public");

    // dispatch
    match args.action {
        // 查订阅
        Action::GetSub { topic } => {
            let subs = DefaultTopicService::new(create_pubsub_client(naming).await?)
                .get_subs(&topic)
                .await?;
            println!("{:?}", subs);
        }
        // 删除订阅
        Action::Unsub {
            client,
            topicfilters,
        } => {
            DefaultTopicService::new(create_pubsub_client(naming).await?)
                .unsub(&client, topicfilters)
                .await?;
            println!("unsub success");
        }
    }
    Ok(())
}

async fn create_pubsub_client(naming: impl Naming) -> Result<PubSubServiceClient<Channel>> {
    Ok(PubSubServiceClient::connect(format!(
        "http://{}",
        get_service_addr(naming, "subscriber").await?
    ))
    .await?)
}

async fn get_service_addr(naming: impl Naming, service: &str) -> Result<String> {
    naming
        .get_instance(service)
        .await?
        .ok_or_else(|| anyhow!("no available {} instance", service))
        .map(|ins| ins.as_addr())
}

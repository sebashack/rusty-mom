use log::{info, warn};
use tonic::transport::Channel;

use crate::messages::message_stream_client::MessageStreamClient;
use crate::messages::{CreateQueueRequest, DeleteQueueRequest};

pub struct Client {
    connection: MessageStreamClient<Channel>,
}

impl Client {
    async fn connect(&self, host: String, port: u16) -> Self {
        println!("Connecting to host on {host}:{port}");
        let addr = format!("{}:{}", host, port);

        Client {
            connection: MessageStreamClient::connect(addr).await.unwrap(),
        }
    }

    async fn create_queue(&mut self, queue_label: String) {
        let req = tonic::Request::new(CreateQueueRequest { queue_label });
        match self.connection.create_queue(req).await {
            Ok(_) => info!("Queue created"),
            Err(err) => warn!("Failed to create queue: {}", err),
        }
    }

    async fn delete_queue(&mut self, queue_label: String) {
        let req = tonic::Request::new(DeleteQueueRequest { queue_label });
        match self.connection.delete_queue(req).await {
            Ok(_) => info!("Queue deleted"),
            Err(err) => warn!("Failed to delete queue: {}", err),
        }
    }
}

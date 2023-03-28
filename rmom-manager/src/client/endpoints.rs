use tonic::transport::Channel;

use crate::messages::message_stream_client::MessageStreamClient;
use crate::messages::{CreateChannelRequest, CreateQueueRequest, DeleteQueueRequest};

pub struct Client {
    connection: MessageStreamClient<Channel>,
}

impl Client {
    pub async fn connect(host: String, port: u16) -> Self {
        println!("Connecting to host on http://{host}:{port}");
        let addr = format!("http://{}:{}", host, port);

        Client {
            connection: MessageStreamClient::connect(addr).await.unwrap(),
        }
    }

    pub async fn create_queue(&mut self, queue_label: String) -> Result<(), String> {
        let req = tonic::Request::new(CreateQueueRequest { queue_label });
        match self.connection.create_queue(req).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to create queue: {}", err)),
        }
    }

    pub async fn delete_queue(&mut self, queue_label: String) -> Result<(), String> {
        let req = tonic::Request::new(DeleteQueueRequest { queue_label });
        match self.connection.delete_queue(req).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to delete queue: {}", err)),
        }
    }

    pub async fn create_channel(
        &mut self,
        queue_label: String,
        topic: String,
    ) -> Result<String, String> {
        let req = tonic::Request::new(CreateChannelRequest { queue_label, topic });
        match self.connection.create_channel(req).await {
            Ok(res) => Ok(res.into_inner().channel_id),
            Err(err) => Err(format!("Failed to create channel: {}", err)),
        }
    }
}

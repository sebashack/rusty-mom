use tonic::transport::Channel;
use tonic::Code;

use crate::messages::message_stream_client::MessageStreamClient;
use crate::messages::{
    CreateChannelRequest, CreateQueueRequest, DeleteChannelRequest, DeleteQueueRequest,
    HeartbeatRequest, RebuildQueueRequest,
};

pub struct Client {
    connection: MessageStreamClient<Channel>,
}

impl Client {
    pub async fn connect(host: String, port: i32) -> Option<Self> {
        println!("Connecting to host on http://{host}:{port}");
        let addr = format!("http://{}:{}", host, port);

        match MessageStreamClient::connect(addr).await {
            Ok(connection) => Some(Client { connection }),
            Err(_) => None,
        }
    }

    pub async fn get_heartbeat(&mut self) -> Result<(), Code> {
        let req = tonic::Request::new(HeartbeatRequest {});
        match self.connection.get_heartbeat(req).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.code()),
        }
    }

    pub async fn create_queue(&mut self, queue_label: &str) -> Result<(), String> {
        let req = tonic::Request::new(CreateQueueRequest {
            queue_label: queue_label.to_string(),
        });
        match self.connection.create_queue(req).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to create queue: {}", err)),
        }
    }

    pub async fn rebuild_queue(&mut self, queue_label: &str) -> Result<(), String> {
        let req = tonic::Request::new(RebuildQueueRequest {
            queue_label: queue_label.to_string(),
        });
        match self.connection.rebuild_queue(req).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to rebuild queue: {}", err)),
        }
    }

    pub async fn delete_queue(&mut self, queue_label: &str) -> Result<(), String> {
        let req = tonic::Request::new(DeleteQueueRequest {
            queue_label: queue_label.to_string(),
        });
        match self.connection.delete_queue(req).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to delete queue: {}", err)),
        }
    }

    pub async fn delete_channel(&mut self, channel_id: &str) -> Result<(), String> {
        let req = tonic::Request::new(DeleteChannelRequest {
            channel_id: channel_id.to_string(),
        });
        match self.connection.delete_channel(req).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to delete channel: {}", err)),
        }
    }

    pub async fn create_channel(
        &mut self,
        queue_label: &str,
        topic: &str,
    ) -> Result<String, String> {
        let req = tonic::Request::new(CreateChannelRequest {
            queue_label: queue_label.to_string(),
            topic: topic.to_string(),
        });
        match self.connection.create_channel(req).await {
            Ok(res) => Ok(res.into_inner().channel_id),
            Err(err) => Err(format!("Failed to create channel: {}", err)),
        }
    }
}

use futures_lite::stream::{Stream, StreamExt};
use log::info;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Code, Request, Response, Status};
use uuid::Uuid;

use super::queue::{BroadcastEnd, ChannelId, ChannelReceiver, Queue, QueueLabel};
use crate::messages::message_stream_server::{MessageStream, MessageStreamServer};
use crate::messages::{
    CreateQueueOkResponse, CreateQueueRequest, Message, Push, PushOkResponse, SubscriptionRequest, DeleteQueueOkResponse, DeleteQueueRequest,
};

pub type ChannelStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

pub struct StreamServer {
    host: String,
    port: u16,
    channel_receivers: Arc<Mutex<HashMap<ChannelId, ChannelReceiver>>>,
    broadcast_ends: Arc<Mutex<HashMap<QueueLabel, (Queue, BroadcastEnd)>>>,
    buffer_size: usize,
}

#[tonic::async_trait]
impl MessageStream for StreamServer {
    type SubscribeToChannelStream = ChannelStream;

    async fn subscribe_to_channel(
        &self,
        req: Request<SubscriptionRequest>,
    ) -> Result<Response<Self::SubscribeToChannelStream>, Status> {
        let req = req.into_inner();
        info!("Request to SUBSCRIBE to channel {0}", req.channel_id);

        if let Ok(chan_id) = Uuid::parse_str(req.channel_id.as_str()) {
            let mut lock = self.channel_receivers.lock().unwrap();
            let chan_receiver = lock.remove(&chan_id);
            drop(lock);

            if let Some(chan_receiver) = chan_receiver {
                let stream = chan_receiver.receiver.filter(move |msg| {
                    if let Some(chan_topic) = &chan_receiver.topic {
                        if let Ok(msg) = msg {
                            return chan_topic.as_str() == msg.topic.as_str();
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                });

                Ok(Response::new(Box::pin(stream)))
            } else {
                Err(Status::new(Code::NotFound, "Channel not found"))
            }
        } else {
            Err(Status::new(
                Code::InvalidArgument,
                "Invalid channel_id: not a uuid v4",
            ))
        }
    }

    async fn push_to_channel(
        &self,
        push: Request<Push>,
    ) -> Result<Response<PushOkResponse>, Status> {
        let push = push.into_inner();
        info!("Request to PUSH");

        let lock = self.broadcast_ends.lock().unwrap();
        if let Some((_, broadcast_end)) = lock.get(&push.queue_label) {
            let message = Message {
                id: Uuid::new_v4().to_string(),
                content: push.content,
                topic: push.topic,
            };

            match broadcast_end.broadcast(message) {
                Ok(()) => Ok(Response::new(PushOkResponse {})),
                Err(msg) => Err(Status::new(Code::Internal, msg)),
            }
        } else {
            Err(Status::new(Code::NotFound, "Queue not found"))
        }
    }

    async fn create_queue(
        &self,
        request: Request<CreateQueueRequest>,
    ) -> Result<Response<CreateQueueOkResponse>, Status> {
        let label = request.into_inner().queue_label;
        info!("Request to CREATE queue with label: {}", label);

        let mut be_map = self.broadcast_ends.lock().unwrap();
        
        if be_map.contains_key(&label){
            return Err(Status::new(
                Code::InvalidArgument,
                "Queue already exists with this label",
            ))
        }

        let queue = Queue::new(self.buffer_size, label.clone());
        let broadcast_end = queue.get_broadcast_end();
        be_map.insert(label.clone(), (queue, broadcast_end));
        
        info!("Queue with label: {} created succesfully", label);
        Ok(Response::new(CreateQueueOkResponse{}))
    }

    async fn delete_queue(
        &self,
        request: Request<DeleteQueueRequest>,
    ) -> Result<Response<DeleteQueueOkResponse>, Status> {
        let label = request.into_inner().queue_label;
        info!("Request to DELETE queue with label: {}", label);

        let mut broadcast_ends = self.broadcast_ends.lock().unwrap();

        match broadcast_ends.remove(&label) {
            Some(_) => {
                info!("Queue with label: {} deleted succesfully", label);
                Ok(Response::new(DeleteQueueOkResponse{}))
            }
            None => Err(Status::new(
                Code::InvalidArgument,
                "Queue not found with the given label",
            )),
        }
    }
}

impl StreamServer {
    pub fn new(host: String, port: u16, buffer_size: usize) -> Self {
        let channel_receivers = Arc::new(Mutex::new(HashMap::new()));
        let broadcast_ends = Arc::new(Mutex::new(HashMap::new()));

        StreamServer {
            host,
            port,
            channel_receivers,
            broadcast_ends,
            buffer_size,
        }
    }

    pub async fn run(self) {
        let addr = format!("{}:{}", self.host, self.port);

        println!("Running stream server on {addr}");
        Server::builder()
            .add_service(MessageStreamServer::new(self))
            .serve(addr.to_socket_addrs().unwrap().next().unwrap())
            .await
            .unwrap();
    }
}

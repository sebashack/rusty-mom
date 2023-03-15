use futures_lite::Stream;
use log::info;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Code, Request, Response, Status};
use uuid::Uuid;

use super::queue::{ChannelId, ChannelReceiver, ChannelSender};
use crate::messages::message_stream_server::{MessageStream, MessageStreamServer};
use crate::messages::{Message, Push, PushOkResponse, SubscriptionRequest};

#[derive(Default)]
pub struct StreamServer {
    channel_receivers: Arc<Mutex<HashMap<ChannelId, ChannelReceiver>>>,
    channel_senders: Arc<Mutex<HashMap<ChannelId, ChannelSender>>>,
}

pub type ChannelStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

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
                let stream = Box::pin(chan_receiver.receiver);

                Ok(Response::new(stream as Self::SubscribeToChannelStream))
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
        info!("Request to PUSH to channel {0}", push.channel_id);

        if let Ok(chan_id) = Uuid::parse_str(push.channel_id.as_str()) {
            let lock = self.channel_senders.lock().unwrap();
            let chan_sender = lock.get(&chan_id);

            if let Some(chan_sender) = chan_sender {
                let message = Message {
                    id: Uuid::new_v4().to_string(),
                    content: push.content,
                    topic: push.topic,
                };

                match chan_sender.broadcast(message) {
                    Ok(()) => {
                        drop(lock);
                        Ok(Response::new(PushOkResponse {}))
                    }
                    Err(msg) => {
                        drop(lock);
                        Err(Status::new(Code::Internal, msg))
                    }
                }
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
}

pub async fn run_stream_server(host: String, port: u16) {
    let server = StreamServer {
        channel_receivers: Arc::new(Mutex::new(HashMap::new())),
        channel_senders: Arc::new(Mutex::new(HashMap::new())),
    };
    let addr = format!("{host}:{port}");

    println!("Running stream server on {addr}");
    Server::builder()
        .add_service(MessageStreamServer::new(server))
        .serve(addr.to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
}

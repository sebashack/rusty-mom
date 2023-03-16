use async_broadcast::Receiver;
use log::info;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Code, Request, Response, Status};
use uuid::Uuid;

use super::queue::{BroadcastEnd, ChannelId, ChannelReceiver};
use crate::messages::message_stream_server::{MessageStream, MessageStreamServer};
use crate::messages::{Message, Push, PushOkResponse, SubscriptionRequest};

pub type ChannelStream = Pin<Box<Receiver<Result<Message, Status>>>>;

pub struct StreamServer {
    channel_receivers: Arc<Mutex<HashMap<ChannelId, ChannelReceiver>>>,
    broadcast_end: Arc<Mutex<BroadcastEnd>>,
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
                Ok(Response::new(Box::pin(chan_receiver.receiver)))
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
            let chan_sender = self.broadcast_end.lock().unwrap();
            let message = Message {
                id: Uuid::new_v4().to_string(),
                content: push.content,
                topic: push.topic,
            };

            match chan_sender.broadcast(message) {
                Ok(()) => {
                    drop(chan_sender);
                    Ok(Response::new(PushOkResponse {}))
                }
                Err(msg) => {
                    drop(chan_sender);
                    Err(Status::new(Code::Internal, msg))
                }
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
        broadcast_end: Arc::new(Mutex::new(unimplemented!())),
    };
    let addr = format!("{host}:{port}");

    println!("Running stream server on {addr}");
    Server::builder()
        .add_service(MessageStreamServer::new(server))
        .serve(addr.to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
}

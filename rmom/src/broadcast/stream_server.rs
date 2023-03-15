use async_broadcast::Receiver;
use futures_lite::Stream;
use log::info;
use std::collections::HashMap;
use std::iter;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::{net::ToSocketAddrs, time::Duration};
use tokio_stream::StreamExt;
use tonic::{transport::Server, Code, Request, Response, Status};
use uuid::Uuid;

use super::queue::{ChannelId, ChannelReceiver};
use crate::messages::message_stream_server::{MessageStream, MessageStreamServer};
use crate::messages::{ConnectionRequest, Message};

#[derive(Default)]
pub struct StreamServer {
    channels: Arc<Mutex<HashMap<ChannelId, ChannelReceiver>>>,
}

pub type ChannelStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[tonic::async_trait]
impl MessageStream for StreamServer {
    type ConnectToChannelStream = ChannelStream;

    async fn connect_to_channel(
        &self,
        req: Request<ConnectionRequest>,
    ) -> Result<Response<Self::ConnectToChannelStream>, Status> {
        let req = req.into_inner();
        info!("Received request to connect to Channel {0}", req.channel_id);

        if let Ok(chan_id) = Uuid::parse_str(req.channel_id.as_str()) {
            let mut lock = self.channels.lock().unwrap();
            let chan_receiver = lock.remove(&chan_id);
            drop(lock);

            if let Some(chan_receiver) = chan_receiver {
                let stream = Box::pin(chan_receiver.receiver);

                Ok(Response::new(stream as Self::ConnectToChannelStream))
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
        channels: Arc::new(Mutex::new(HashMap::new())),
    };
    let addr = format!("{host}:{port}");

    println!("Running stream server on {addr}");
    Server::builder()
        .add_service(MessageStreamServer::new(server))
        .serve(addr.to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
}

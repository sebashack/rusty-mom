use log::info;
use std::iter;
use std::{net::ToSocketAddrs, time::Duration};
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use super::queue::ChannelStream;
use crate::messages::message_stream_server::{MessageStream, MessageStreamServer};
use crate::messages::{ConnectionRequest, Message};

#[derive(Debug, Default)]
pub struct StreamServer {}

#[tonic::async_trait]
impl MessageStream for StreamServer {
    type ConnectToChannelStream = ChannelStream;

    async fn connect_to_channel(
        &self,
        req: Request<ConnectionRequest>,
    ) -> Result<Response<Self::ConnectToChannelStream>, Status> {
        let req = req.into_inner();
        info!("Received request to connect to Channel {0}", req.channel_id);
        let messages = iter::repeat_with(|| {
            return Ok(Message {
                id: Uuid::new_v4().to_string(),
                content: vec![1, 2, 3],
                topic: "NOTOPIC".to_string(),
            });
        });

        let stream: ChannelStream =
            Box::pin(tokio_stream::iter(messages).throttle(Duration::from_millis(200)));

        Ok(Response::new(stream as Self::ConnectToChannelStream))
    }
}

pub async fn run_stream_server(host: String, port: u16) {
    let server = StreamServer {};
    let addr = format!("{host}:{port}");

    println!("Running stream server on {addr}");
    Server::builder()
        .add_service(MessageStreamServer::new(server))
        .serve(addr.to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
}

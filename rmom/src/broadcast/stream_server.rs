use futures_lite::Stream;
use log::info;
use std::iter;
use std::{net::ToSocketAddrs, pin::Pin, time::Duration};
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use messages::message_stream_server::{MessageStream, MessageStreamServer};
use messages::{ConnectionRequest, Message};

pub mod messages {
    tonic::include_proto!("messages");
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[derive(Debug, Default)]
pub struct StreamServer {}

#[tonic::async_trait]
impl MessageStream for StreamServer {
    type ConnectToChannelStream = ResponseStream;

    async fn connect_to_channel(
        &self,
        req: Request<ConnectionRequest>,
    ) -> Result<Response<Self::ConnectToChannelStream>, Status> {
        let req = req.into_inner();
        info!("Received request to connect to Channel {0}", req.channel_id);
        let messages = iter::repeat_with(|| {
            return Ok(Message {
                message_id: Uuid::new_v4().to_string(),
                content: vec![1, 2, 3],
                topic: "NOTOPIC".to_string(),
            });
        });

        let stream = Box::pin(tokio_stream::iter(messages).throttle(Duration::from_millis(200)));

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

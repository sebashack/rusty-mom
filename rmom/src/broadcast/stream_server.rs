use std::iter;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use futures_lite::Stream;
use std::{net::ToSocketAddrs, pin::Pin, time::Duration};
use tokio_stream::StreamExt;

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
        _request: Request<ConnectionRequest>,
    ) -> Result<Response<Self::ConnectToChannelStream>, Status> {
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

pub async fn run_stream_server() {
    let server = StreamServer {};
    Server::builder()
        .add_service(MessageStreamServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
}

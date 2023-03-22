use rmom::broadcast::stream_server::StreamServer;

#[tokio::main]
async fn main() {
    env_logger::init();
    let stream_server = StreamServer::new("127.0.0.1".to_string(), 50051);
    stream_server.run().await;
}

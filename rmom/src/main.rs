use rmom::broadcast::queue::Queue;
use rmom::broadcast::stream_server::StreamServer;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, time};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut queue = Queue::new(100, "q1".to_string());

    let broadcast_end = Arc::new(Mutex::new(queue.get_broadcast_end()));
    let channel_receivers = Arc::new(Mutex::new(HashMap::new()));
    let stream_server = StreamServer::new(
        "127.0.0.1".to_string(),
        50051,
        Arc::clone(&broadcast_end),
        Arc::clone(&channel_receivers),
    );

    //let mut chan1 = queue.duplicate_channel();
    //let stream1 = chan1.get_receiver_stream();

    //// Pusher
    //let one_sec = time::Duration::from_millis(1000);

    //thread::spawn(move || loop {
    //    // TODO: Construct a message
    //    let msg = unimplemented!();
    //    chan1.broadcast(msg);
    //    thread::sleep(one_sec);
    //});
    //
    stream_server.run().await;
}

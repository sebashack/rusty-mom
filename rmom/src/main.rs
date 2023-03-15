use rmom::broadcast::queue::Queue;
use rmom::broadcast::stream_server::run_stream_server;
use std::{thread, time};

#[tokio::main]
async fn main() {
    env_logger::init();

    //let mut queue = Queue::new(100, "q1".to_string());

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

    run_stream_server("127.0.0.1".to_string(), 50051).await;
}

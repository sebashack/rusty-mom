use rmom::broadcast::queue::Queue;
use rmom::broadcast::stream_server::StreamServer;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use uuid::Uuid;

use rmom::messages::Message;

#[tokio::main]
async fn main() {
    env_logger::init();

    let four_secs = time::Duration::from_millis(4000);

    // Queues
    let mut queue1 = Queue::new(200, "q1".to_string());
    let broadcast_end_q1 = ("q1".to_string(), queue1.get_broadcast_end());

    let mut queue2 = Queue::new(200, "q2".to_string());
    let broadcast_end_q2 = ("q2".to_string(), queue2.get_broadcast_end());

    let broadcast_ends = Arc::new(Mutex::new(HashMap::from([
        broadcast_end_q1,
        broadcast_end_q2,
    ])));

    // Pusher 1 Q1
    let chan1_q1 = queue1.duplicate_channel(None);
    let lock1_q1 = Arc::clone(&broadcast_ends);

    thread::spawn(move || loop {
        let msg = Message {
            id: Uuid::new_v4().to_string(),
            content: "Hello Q1".into(),
            topic: "__NONE__".to_string(),
        };
        let pointer = lock1_q1.lock().unwrap();
        let endpoint = pointer.get(&"q1".to_string()).unwrap();
        endpoint.broadcast(msg).unwrap();
        drop(pointer);
        thread::sleep(four_secs);
    });

    // Pusher 2 Q1
    let chan2_q1 = queue1.duplicate_channel(Some("A".to_string()));
    let lock2_q1 = Arc::clone(&broadcast_ends);

    thread::spawn(move || loop {
        let msg = Message {
            id: Uuid::new_v4().to_string(),
            content: "Hello Q1 A".into(),
            topic: "A".to_string(),
        };
        let pointer = lock2_q1.lock().unwrap();
        let endpoint = pointer.get(&"q1".to_string()).unwrap();
        endpoint.broadcast(msg).unwrap();
        drop(pointer);
        thread::sleep(four_secs);
    });

    // Pusher 1 Q2
    let chan1_q2 = queue2.duplicate_channel(None);
    let lock1_q2 = Arc::clone(&broadcast_ends);

    thread::spawn(move || loop {
        let msg = Message {
            id: Uuid::new_v4().to_string(),
            content: "Hello Q2".into(),
            topic: "__NONE__".to_string(),
        };
        let pointer = lock1_q2.lock().unwrap();
        let endpoint = pointer.get(&"q2".to_string()).unwrap();
        endpoint.broadcast(msg).unwrap();
        drop(pointer);
        thread::sleep(four_secs);
    });

    println!("Chan1 q1: {}", &chan1_q1.id);
    println!("Chan2 q1: {}", &chan2_q1.id);
    println!("Chan1 q2: {}", &chan1_q2.id);

    let channel_receivers = Arc::new(Mutex::new(HashMap::from([
        (chan1_q1.id.clone(), chan1_q1),
        (chan2_q1.id.clone(), chan2_q1),
        (chan1_q2.id.clone(), chan1_q2),
    ])));

    let stream_server = StreamServer::new(
        "127.0.0.1".to_string(),
        50051,
        Arc::clone(&broadcast_ends),
        Arc::clone(&channel_receivers),
    );

    stream_server.run().await;
}

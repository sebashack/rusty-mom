//use std::{thread, time};
//use uuid::Uuid;

//use rmom::broadcast::queue::{Message, Queue};
use rmom::broadcast::stream_server::run_stream_server;

#[tokio::main]
async fn main() {
    run_stream_server().await;
    //let mut queue = Queue::new(100, "q1".to_string());

    //let chan1 = queue.duplicate_channel(|msg| println!("Chan1: msgid={}", msg.id.to_string()));

    //let one_sec = time::Duration::from_millis(1000);
    //let three_secs = time::Duration::from_millis(3000);

    //// Sender 1
    //thread::spawn(move || loop {
    //    let msg = Message {
    //        id: Uuid::new_v4(),
    //        content: Vec::new(),
    //        topic: None,
    //    };

    //    chan1.broadcast(msg);
    //    thread::sleep(one_sec);
    //});

    //let chan2 = queue.duplicate_channel(|msg| println!("Chan2: msgid={}", msg.id.to_string()));

    //thread::sleep(one_sec);

    //// Sender 2
    //thread::spawn(move || loop {
    //    let msg = Message {
    //        id: Uuid::new_v4(),
    //        content: Vec::new(),
    //        topic: None,
    //    };

    //    chan2.broadcast(msg);
    //    thread::sleep(one_sec);
    //});

    //let chan3 = queue.duplicate_channel(|msg| println!("Chan3: msgid={}", msg.id.to_string()));
    //let chan3_id = chan3.id.clone();

    //thread::sleep(three_secs);

    //// Sender 3
    //thread::spawn(move || loop {
    //    let msg = Message {
    //        id: Uuid::new_v4(),
    //        content: Vec::new(),
    //        topic: None,
    //    };

    //    chan3.broadcast(msg);
    //    thread::sleep(one_sec);
    //});

    //thread::sleep(three_secs);
    //thread::sleep(three_secs);

    //println!("Killing channel 3 receiver ...");

    //queue.kill_channel_thread(&chan3_id);

    //thread::sleep(three_secs);
    //thread::sleep(three_secs);
    //thread::sleep(three_secs);
    //thread::sleep(three_secs);

    //println!("Destroying queue ...");
    //queue.destroy();

    //loop {
    //    thread::sleep(one_sec);
    //    println!("Doing nothing ...")
    //}
}

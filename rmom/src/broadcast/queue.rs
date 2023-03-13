use async_broadcast::{broadcast, Receiver, Sender};
use futures_lite::future::block_on;
use log::warn;
use std::collections::HashMap;
use std::sync::mpsc::{self};
use std::thread;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Message {
    pub id: Uuid,
    pub content: Vec<u8>,
    pub topic: Option<String>,
}

pub struct Queue {
    w: Sender<Message>,
    r: Receiver<Message>,
    channel_threads: HashMap<Uuid, mpsc::Sender<()>>,
}

pub struct Channel {
    pub id: Uuid,
    sender: Sender<Message>,
}

impl Queue {
    pub fn new(k: usize) -> Queue {
        let (w, r) = broadcast(k);
        Queue {
            w,
            r,
            channel_threads: HashMap::new(),
        }
    }

    pub fn duplicate_channel(&mut self, f: impl Fn(Message) + Send + 'static) -> Channel {
        let mut receiver = self.r.clone();
        let (send_kill_signal, receive_kill_signal) = mpsc::channel();

        thread::spawn(move || {
            block_on(async {
                loop {
                    if let Ok(()) = receive_kill_signal.try_recv() {
                        break;
                    } else {
                        if let Ok(msg) = receiver.recv().await {
                            f(msg)
                        } else {
                            warn!("Failed to receive message");
                        }
                    }
                }
            });
        });

        let chan = Channel {
            id: Uuid::new_v4(),
            sender: self.w.clone(),
        };

        self.channel_threads
            .insert(chan.id.clone(), send_kill_signal);

        chan
    }

    pub fn kill_channel_thread(&mut self, chan_id: &Uuid) {
        self.channel_threads.remove(chan_id).map(|kill_sig| {
            if let Err(_) = kill_sig.send(()) {
                warn!("Failed to send kill signal to channel thread")
            }
        });
    }

    pub fn destroy(self) {
        self.w.close();
        self.r.close();
        for (_, kill_sig) in self.channel_threads {
            if let Err(_) = kill_sig.send(()) {
                warn!("Failed to send kill signal to channel thread")
            }
        }
    }
}

impl Channel {
    pub fn broadcast(&self, msg: Message) {
        if let Err(err) = self.sender.try_broadcast(msg) {
            warn!("Failed to broadcast message: {err}");
        }
    }

    pub fn close(self) -> bool {
        self.sender.close()
    }
}

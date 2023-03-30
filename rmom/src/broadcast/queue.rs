use async_broadcast::{broadcast, Receiver, Sender};
use log::warn;
use tonic::Status;
use uuid::Uuid;

use crate::messages::Message;

pub type ChannelId = Uuid;
pub type QueueLabel = String;

pub struct Queue {
    label: QueueLabel,
    w: Sender<Result<Message, Status>>,
    r: Receiver<Result<Message, Status>>,
}

pub struct ChannelReceiver {
    pub id: ChannelId,
    pub topic: Option<String>,
    pub receiver: Receiver<Result<Message, Status>>,
}

pub struct BroadcastEnd {
    sender: Sender<Result<Message, Status>>,
}

impl Queue {
    pub fn new(k: usize, label: String) -> Queue {
        let (w, mut r) = broadcast(k);
        r.set_overflow(true);

        Queue { label, w, r }
    }

    pub fn get_label(&self) -> &str {
        return self.label.as_str();
    }

    pub fn duplicate_channel(&mut self, topic: Option<String>) -> ChannelReceiver {
        let mut receiver = self.r.clone();
        receiver.set_overflow(true);

        let chan_receiver = ChannelReceiver {
            id: Uuid::new_v4(),
            topic,
            receiver: receiver,
        };

        chan_receiver
    }

    pub fn get_broadcast_end(&self) -> BroadcastEnd {
        BroadcastEnd {
            sender: self.w.clone(),
        }
    }

    pub fn destroy(self) {
        self.w.close();
        self.r.close();
    }
}

impl BroadcastEnd {
    pub fn broadcast(&self, msg: Message) -> Result<(), String> {
        if let Err(err) = self.sender.try_broadcast(Ok(msg)) {
            warn!("Failed to broadcast message: {err}");
            Err("Broadcast error".to_string())
        } else {
            Ok(())
        }
    }

    pub fn close(self) -> bool {
        self.sender.close()
    }
}

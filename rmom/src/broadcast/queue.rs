use async_broadcast::{broadcast, Receiver, Sender};
use log::warn;
use tonic::Status;
use uuid::Uuid;

use crate::messages::Message;

pub type ChannelId = Uuid;

pub struct Queue {
    label: String,
    w: Sender<Result<Message, Status>>,
    r: Receiver<Result<Message, Status>>,
}

pub struct ChannelReceiver {
    pub id: ChannelId,
    pub receiver: Receiver<Result<Message, Status>>,
}

pub struct ChannelSender {
    pub id: ChannelId,
    sender: Sender<Result<Message, Status>>,
}

impl Queue {
    pub fn new(k: usize, label: String) -> Queue {
        let (w, r) = broadcast(k);
        Queue { label, w, r }
    }

    pub fn get_label(&self) -> &str {
        return self.label.as_str();
    }

    pub fn duplicate_channel(&mut self) -> (ChannelReceiver, ChannelSender) {
        let chan_sender = ChannelSender {
            id: Uuid::new_v4(),
            sender: self.w.clone(),
        };

        let chan_receiver = ChannelReceiver {
            id: Uuid::new_v4(),
            receiver: self.r.clone(),
        };

        (chan_receiver, chan_sender)
    }

    pub fn destroy(self) {
        self.w.close();
        self.r.close();
    }
}

impl ChannelSender {
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

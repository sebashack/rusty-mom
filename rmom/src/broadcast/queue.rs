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

pub type ChannelReceiver = Receiver<Result<Message, Status>>;

pub struct Channel {
    pub id: ChannelId,
    sender: Sender<Result<Message, Status>>,
    //receiver: Option<ChannelReceiver>,
}

impl Queue {
    pub fn new(k: usize, label: String) -> Queue {
        let (w, r) = broadcast(k);
        Queue { label, w, r }
    }

    pub fn get_label(&self) -> &str {
        return self.label.as_str();
    }

    pub fn duplicate_channel(&mut self) -> Channel {
        let chan = Channel {
            id: Uuid::new_v4(),
            sender: self.w.clone(),
            //receiver: Some(self.r.clone()),
        };

        chan
    }

    pub fn destroy(self) {
        self.w.close();
        self.r.close();
    }
}

impl Channel {
    pub fn broadcast(&self, msg: Message) {
        if let Err(err) = self.sender.try_broadcast(Ok(msg)) {
            warn!("Failed to broadcast message: {err}");
        }
    }

    pub fn close(self) -> bool {
        self.sender.close()
    }
}

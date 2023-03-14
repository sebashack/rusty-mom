use async_broadcast::{broadcast, Receiver, Sender};
use futures_lite::Stream;
use log::warn;
use uuid::Uuid;

use std::pin::Pin;
use tonic::Status;

use crate::messages::Message;

pub type ChannelStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

pub struct Queue {
    label: String,
    w: Sender<Result<Message, Status>>,
    r: Receiver<Result<Message, Status>>,
}

type ChannelReceiver = Receiver<Result<Message, Status>>;

pub struct Channel {
    pub id: Uuid,
    sender: Sender<Result<Message, Status>>,
    receiver: Option<ChannelReceiver>,
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
            receiver: Some(self.r.clone()),
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

    pub fn get_receiver_stream(&mut self) -> Option<ChannelStream> {
        if let Some(r) = self.receiver.take() {
            Some(Box::pin(r))
        } else {
            None
        }
    }

    pub fn close(self) -> bool {
        self.sender.close()
    }
}

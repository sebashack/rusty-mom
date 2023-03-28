use crate::messages::message_stream_client::MessageStreamClient;

pub struct Client<T> {
    host: String,
    port: u16,
    connection: MessageStreamClient<T>,
}

impl<T> Client<T> {
    // TODO: Implement
    // new(host, port) -> Client
    // connect(self) -> MessageStreamClient<T>
    // create_queue(self, req: CreateQueueRequest) -> Result<(), String>
    // Make a request in main.rs
}

use futures::lock::Mutex;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Route, State};
use std::sync::Arc;

use crate::client::endpoints::Client;

pub struct AvailableClients {
    pub clients: Arc<Mutex<Client>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ChannelInfo {
    pub id: String,
    pub host: String,
    pub port: u16,
}

#[post("/queues/<label>")]
async fn post_queue(
    state: &State<AvailableClients>,
    label: String,
) -> Result<(), (Status, String)> {
    let mut client = state.clients.lock().await;
    let response = client.create_queue(label).await;

    match response {
        Ok(_) => Ok(()),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

#[delete("/queues/<label>")]
async fn delete_queue(
    state: &State<AvailableClients>,
    label: String,
) -> Result<(), (Status, String)> {
    let mut client = state.clients.lock().await;
    let response = client.delete_queue(label).await;

    match response {
        Ok(_) => Ok(()),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

#[put("/queues/<label>/channels/<topic>", format = "json")]
async fn put_channel(
    state: &State<AvailableClients>,
    label: String,
    topic: String,
) -> Result<Json<ChannelInfo>, (Status, String)> {
    let mut client = state.clients.lock().await;
    let response = client.create_channel(label, topic).await;

    // TODO: Unharcode host and port when we keep track of
    // available clients.
    match response {
        Ok(channel_id) => Ok(Json(ChannelInfo {
            id: channel_id,
            host: "127.0.0.1".to_string(),
            port: 50051,
        })),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

pub fn endpoints() -> Vec<Route> {
    routes![post_queue, delete_queue, put_channel]
}

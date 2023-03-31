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
pub struct ConnectionInfo {
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

#[get("/queues")]
async fn get_queues(
    state: &State<AvailableClients>,
) -> Result<Json<Vec<String>>, (Status, String)> {
    let mut client = state.clients.lock().await;
    let response = client.list_queues().await;

    match response {
        Ok(queues) => Ok(Json(queues)),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

#[get("/channels")]
async fn get_channels(
    state: &State<AvailableClients>,
) -> Result<Json<Vec<String>>, (Status, String)> {
    let mut client = state.clients.lock().await;
    let response = client.list_channels().await;

    match response {
        Ok(queues) => Ok(Json(queues)),

#[delete("/channels/<channel_id>")]
async fn delete_channel(
    state: &State<AvailableClients>,
    channel_id: String,
) -> Result<(), (Status, String)> {
    let mut client = state.clients.lock().await;
    let response = client.delete_channel(channel_id).await;

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
) -> Result<Json<ConnectionInfo>, (Status, String)> {
    let mut client = state.clients.lock().await;
    let response = client.create_channel(label, topic).await;

    // TODO: Unhardcode host and port when we keep track of
    // available clients.
    match response {
        Ok(channel_id) => Ok(Json(ConnectionInfo {
            id: channel_id,
            host: "127.0.0.1".to_string(),
            port: 50051,
        })),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

pub fn endpoints() -> Vec<Route> {
    routes![
        post_queue,
        delete_queue,
        put_channel,
        get_queues,
        get_channels,
        delete_channel
    ]
}

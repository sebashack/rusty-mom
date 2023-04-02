use futures::lock::Mutex;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Route, State};
use std::collections::HashMap;
use std::sync::Arc;

use crate::client::endpoints::Client;

pub struct RegisteredMoM {
    pub connection: Option<Client>,
    pub host: String,
    pub port: i32,
}

pub struct RegisteredMoMs {
    pub moms: Arc<Mutex<HashMap<(String, i32), RegisteredMoM>>>,
}

// TODO: Remove this hardcoded host and implement logic to decide which moms to pick out.
const HARCODED_HOST: &str = "127.0.0.1";
const HARCODED_PORT: i32 = 50051;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ConnectionInfo {
    pub id: String,
    pub host: String,
    pub port: i32,
}

#[post("/queues/<label>")]
async fn post_queue(state: &State<RegisteredMoMs>, label: String) -> Result<(), (Status, String)> {
    let mut lock = state.moms.lock().await;
    let client = lock
        .get_mut(&(HARCODED_HOST.to_string(), HARCODED_PORT))
        .unwrap()
        .connection
        .as_mut()
        .unwrap();

    let response = client.create_queue(label).await;
    match response {
        Ok(_) => Ok(()),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

#[delete("/queues/<label>")]
async fn delete_queue(
    state: &State<RegisteredMoMs>,
    label: String,
) -> Result<(), (Status, String)> {
    let mut lock = state.moms.lock().await;
    let client = lock
        .get_mut(&(HARCODED_HOST.to_string(), HARCODED_PORT))
        .unwrap()
        .connection
        .as_mut()
        .unwrap();

    let response = client.delete_queue(label).await;
    match response {
        Ok(_) => Ok(()),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

#[get("/queues")]
async fn get_queues(state: &State<RegisteredMoMs>) -> Result<Json<Vec<String>>, (Status, String)> {
    let mut lock = state.moms.lock().await;
    let client = lock
        .get_mut(&(HARCODED_HOST.to_string(), HARCODED_PORT))
        .unwrap()
        .connection
        .as_mut()
        .unwrap();

    let response = client.list_queues().await;
    match response {
        Ok(queues) => Ok(Json(queues)),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

#[get("/channels")]
async fn get_channels(
    state: &State<RegisteredMoMs>,
) -> Result<Json<Vec<String>>, (Status, String)> {
    let mut lock = state.moms.lock().await;
    let client = lock
        .get_mut(&(HARCODED_HOST.to_string(), HARCODED_PORT))
        .unwrap()
        .connection
        .as_mut()
        .unwrap();

    let response = client.list_channels().await;
    match response {
        Ok(queues) => Ok(Json(queues)),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

#[delete("/channels/<channel_id>")]
async fn delete_channel(
    state: &State<RegisteredMoMs>,
    channel_id: String,
) -> Result<(), (Status, String)> {
    let mut lock = state.moms.lock().await;
    let client = lock
        .get_mut(&(HARCODED_HOST.to_string(), HARCODED_PORT))
        .unwrap()
        .connection
        .as_mut()
        .unwrap();

    let response = client.delete_channel(channel_id).await;
    match response {
        Ok(_) => Ok(()),
        Err(err) => Err((Status::BadRequest, err)),
    }
}

#[put("/queues/<label>/channels/<topic>", format = "json")]
async fn put_channel(
    state: &State<RegisteredMoMs>,
    label: String,
    topic: String,
) -> Result<Json<ConnectionInfo>, (Status, String)> {
    let mut lock = state.moms.lock().await;
    let client = lock
        .get_mut(&(HARCODED_HOST.to_string(), HARCODED_PORT))
        .unwrap()
        .connection
        .as_mut()
        .unwrap();

    let response = client.create_channel(label, topic).await;
    match response {
        Ok(channel_id) => Ok(Json(ConnectionInfo {
            id: channel_id,
            host: HARCODED_HOST.to_string(),
            port: HARCODED_PORT,
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

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Route, State};

use crate::api::mom::AvailableMoMs;
use crate::database::connection::DbConnection;
use crate::database::crud;

// TODO: Remove this hardcoded host and implement logic to decide which moms to pick out.
const HARCODED_HOST: &str = "127.0.0.1";
const HARCODED_PORT: i32 = 50051;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ChannelInfo {
    pub id: Uuid,
    pub host: String,
    pub topic: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct QueueInfo {
    pub label: String,
    pub host: String,
    pub port: i32,
}

#[post("/queues/<label>")]
async fn post_queue(
    mut db: DbConnection,
    state: &State<AvailableMoMs>,
    label: String,
) -> Result<(), (Status, String)> {
    let label = label.to_lowercase();

    if crud::select_if_queue_exists(&mut db, label.as_str()).await {
        Err((Status::BadRequest, "Queue already exists".to_string()))
    } else {
        if let Some((key, mom_id)) = AvailableMoMs::get_random_up_key(&mut db).await {
            let mut lock = state.moms.lock().await;
            let client = lock.get_mut(&key).unwrap().connection.as_mut().unwrap();

            match client.create_queue(label.as_str()).await {
                Ok(_) => {
                    let queue_id = Uuid::new_v4();
                    crud::insert_queue(&mut db, &queue_id, label.as_str(), &mom_id).await;
                    Ok(())
                }
                Err(err) => Err((Status::BadRequest, err)),
            }
        } else {
            Err((Status::InternalServerError, "No MoMs available".to_string()))
        }
    }
}

// TODO: On deleting queues we also have messages
#[delete("/queues/<label>")]
async fn delete_queue(
    mut db: DbConnection,
    state: &State<AvailableMoMs>,
    label: String,
) -> Result<(), (Status, String)> {
    if let Some(queue_record) = crud::select_queue(&mut db, label.as_str()).await {
        if queue_record.mom_id.is_none() {
            return Err((Status::NotFound, "MoM not available".to_string()));
        }

        if let Some(mom_record) = crud::select_mom(&mut db, &queue_record.mom_id.unwrap()).await {
            let key = (mom_record.host, mom_record.port);
            let mut lock = state.moms.lock().await;
            let client = lock.get_mut(&key).unwrap().connection.as_mut().unwrap();

            crud::delete_queue(&mut db, &queue_record.id).await;
            match client.delete_queue(label.as_str()).await {
                Ok(_) => Ok(()),
                Err(err) => Err((Status::BadRequest, err)),
            }
        } else {
            Err((Status::InternalServerError, "MoM not available".to_string()))
        }
    } else {
        Err((Status::NotFound, "Queue not found".to_string()))
    }
}

#[get("/queues")]
async fn get_queues(mut db: DbConnection) -> Json<Vec<String>> {
    let records = crud::select_all_queues(&mut db).await;
    Json(records.into_iter().map(|q| q.label).collect())
}

#[get("/queue/<queue_label>/topics")]
async fn get_queue_topics(mut db: DbConnection, queue_label: &str) -> Json<Vec<String>> {
    let records = crud::select_all_topics_by_queue_label(&mut db, queue_label).await;
    Json(records.into_iter().map(|t| t.topic).collect())
}

#[get("/channels")]
async fn get_channels(mut db: DbConnection) -> Json<Vec<Uuid>> {
    let records = crud::select_all_channels(&mut db).await;
    Json(records.into_iter().map(|c| c.id).collect())
}

#[get("/queues/<queue_id>")]
async fn get_queue_info(
    mut db: DbConnection,
    queue_id: Uuid,
) -> Result<Json<QueueInfo>, (Status, String)> {
    if let Some(queue) = crud::select_queue_info(&mut db, &queue_id).await {
        Ok(Json(queue))
    } else {
        Err((Status::NotFound, "Queue not found".to_string()))
    }
}

#[get("/channels/<channel_id>")]
async fn get_channel_info(
    mut db: DbConnection,
    channel_id: Uuid,
) -> Result<Json<ChannelInfo>, (Status, String)> {
    if let Some(channel) = crud::select_channel_info(&mut db, &channel_id).await {
        Ok(Json(channel))
    } else {
        Err((Status::NotFound, "Channel not found".to_string()))
    }
}

#[delete("/channels/<channel_id>")]
async fn delete_channel(
    mut db: DbConnection,
    state: &State<AvailableMoMs>,
    channel_id: &str,
) -> Result<(), (Status, String)> {
    let id = Uuid::parse_str(channel_id).unwrap();
    if let Some(channel_record) = crud::select_channel(&mut db, &id).await {
        if let Some(queue_record) =
            crud::select_queue_by_id(&mut db, &channel_record.queue_id).await
        {
            if queue_record.mom_id.is_none() {
                return Err((Status::NotFound, "MoM not available".to_string()));
            }

            if let Some(mom_record) = crud::select_mom(&mut db, &queue_record.mom_id.unwrap()).await
            {
                let key = (mom_record.host, mom_record.port);
                let mut lock = state.moms.lock().await;
                let client = lock.get_mut(&key).unwrap().connection.as_mut().unwrap();

                crud::delete_channel(&mut db, &id).await;
                match client.delete_channel(channel_id).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err((Status::BadRequest, err)),
                }
            } else {
                Err((Status::InternalServerError, "MoM not available".to_string()))
            }
        } else {
            Err((
                Status::InternalServerError,
                "Queue not available".to_string(),
            ))
        }
    } else {
        Err((Status::NotFound, "Channel not found".to_string()))
    }
}

#[put("/queues/<label>/channels/<topic>", format = "json")]
async fn put_channel(
    mut db: DbConnection,
    state: &State<AvailableMoMs>,
    label: String,
    topic: String,
) -> Result<Json<ChannelInfo>, (Status, String)> {
    let topic = topic.to_lowercase();
    let label = label.to_lowercase();

    if let Some(queue_record) = crud::select_queue(&mut db, label.as_str()).await {
        if queue_record.mom_id.is_none() {
            return Err((Status::NotFound, "MoM not available".to_string()));
        }

        if let Some(mom_record) = crud::select_mom(&mut db, &queue_record.mom_id.unwrap()).await {
            let key = (mom_record.host.clone(), mom_record.port);
            let mut lock = state.moms.lock().await;
            let client = lock.get_mut(&key).unwrap().connection.as_mut().unwrap();

            match client.create_channel(label.as_str(), topic.as_str()).await {
                Ok(channel_id) => {
                    let chan_uuid = Uuid::parse_str(channel_id.as_str()).unwrap();
                    crud::insert_channel(&mut db, &chan_uuid, &queue_record.id, topic.as_str())
                        .await;
                    Ok(Json(ChannelInfo {
                        id: chan_uuid,
                        host: mom_record.host,
                        port: mom_record.port,
                        topic,
                    }))
                }
                Err(err) => Err((Status::BadRequest, err)),
            }
        } else {
            Err((Status::InternalServerError, "MoM not available".to_string()))
        }
    } else {
        Err((Status::NotFound, "Queue not found".to_string()))
    }
}

pub fn endpoints() -> Vec<Route> {
    routes![
        delete_channel,
        delete_queue,
        get_channel_info,
        get_channels,
        get_queue_info,
        get_queue_topics,
        get_queues,
        post_queue,
        put_channel,
    ]
}

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{Route, State};

use crate::database::connection::DbConnection;
use crate::database::crud::{self, ChannelInfo, QueueInfo};
use crate::manager::mom::AvailableMoMs;

#[post("/queues/<queue_label>")]
async fn post_queue(
    mut db: DbConnection,
    state: &State<AvailableMoMs>,
    queue_label: String,
) -> Result<(), (Status, String)> {
    let queue_label = queue_label.to_lowercase();

    if crud::select_if_queue_exists(&mut db, queue_label.as_str()).await {
        Err((Status::BadRequest, "Queue already exists".to_string()))
    } else if let Some((key, mom_id)) = AvailableMoMs::get_random_up_key(&mut db).await {
        let mut lock = state.acquire(&key).await;
        let client = lock.as_mut().unwrap().get_client().unwrap();

        match client.create_queue(queue_label.as_str()).await {
            Ok(_) => {
                let queue_id = Uuid::new_v4();
                crud::insert_queue(&mut db, &queue_id, queue_label.as_str(), &mom_id).await;
                Ok(())
            }
            Err(err) => Err((Status::BadRequest, err)),
        }
    } else {
        Err((Status::InternalServerError, "No MoMs available".to_string()))
    }
}

#[delete("/queues/<queue_label>")]
async fn delete_queue(
    mut db: DbConnection,
    state: &State<AvailableMoMs>,
    queue_label: String,
) -> Result<(), (Status, String)> {
    if let Some(queue_record) = crud::select_queue(&mut db, queue_label.as_str()).await {
        if queue_record.mom_id.is_none() {
            return Err((Status::NotFound, "MoM not available".to_string()));
        }

        if let Some(mom_record) = crud::select_mom(&mut db, &queue_record.mom_id.unwrap()).await {
            let key = (mom_record.host, mom_record.port);
            let mut lock = state.acquire(&key).await;
            let client = lock.as_mut().unwrap().get_client().unwrap();

            crud::delete_queue(&mut db, &queue_record.id).await;
            match client.delete_queue(queue_label.as_str()).await {
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
    Json(crud::select_all_topics_by_queue_label(&mut db, queue_label).await)
}

#[get("/channels")]
async fn get_channels(mut db: DbConnection) -> Json<Vec<Uuid>> {
    let records = crud::select_all_channels(&mut db).await;
    Json(records.into_iter().map(|c| c.id).collect())
}

#[get("/queues/<queue_label>")]
async fn get_queue_info(
    mut db: DbConnection,
    queue_label: String,
) -> Result<Json<QueueInfo>, (Status, String)> {
    if let Some(queue) = crud::select_queue_info(&mut db, &queue_label).await {
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
    channel_id: String,
) -> Result<(), (Status, String)> {
    let chan_uuid = Uuid::parse_str(channel_id.as_str()).unwrap();
    if let Some(channel_record) = crud::select_channel(&mut db, &chan_uuid).await {
        if let Some(queue_record) =
            crud::select_queue_by_id(&mut db, &channel_record.queue_id).await
        {
            if queue_record.mom_id.is_none() {
                return Err((Status::NotFound, "MoM not available".to_string()));
            }

            if let Some(mom_record) = crud::select_mom(&mut db, &queue_record.mom_id.unwrap()).await
            {
                let key = (mom_record.host, mom_record.port);
                let mut lock = state.acquire(&key).await;
                let client = lock.as_mut().unwrap().get_client().unwrap();

                crud::delete_channel(&mut db, &chan_uuid).await;
                match client.delete_channel(channel_id.as_str()).await {
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

#[put("/queues/<queue_label>/channels/<topic>", format = "json")]
async fn put_channel(
    mut db: DbConnection,
    state: &State<AvailableMoMs>,
    queue_label: String,
    topic: String,
) -> Result<Json<ChannelInfo>, (Status, String)> {
    let topic = topic.to_lowercase();
    let queue_label = queue_label.to_lowercase();

    if let Some(queue_record) = crud::select_queue(&mut db, queue_label.as_str()).await {
        if queue_record.mom_id.is_none() {
            return Err((Status::NotFound, "MoM not available".to_string()));
        }

        if let Some(mom_record) = crud::select_mom(&mut db, &queue_record.mom_id.unwrap()).await {
            let key = (mom_record.host.clone(), mom_record.port);
            let mut lock = state.acquire(&key).await;
            let client = lock.as_mut().unwrap().get_client().unwrap();

            match client
                .create_channel(queue_label.as_str(), topic.as_str())
                .await
            {
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

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::tokio::{task, time};
use rocket::{Build, Request, Rocket};
use rocket_db_pools::Database;
use std::collections::HashMap;

use super::endpoints::endpoints;
use crate::client::endpoints::Client;
use crate::database::connection::Db;
use crate::database::crud;
use crate::manager::mom::{AvailableMoMs, RegisteredMoM};

#[get("/status")]
pub async fn status() -> &'static str {
    "Ok"
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Internal server error"
}

#[catch(400)]
fn bad_request(req: &Request) -> String {
    format!("Bad request '{}'", req.uri())
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Resource not found '{}'", req.uri())
}

#[catch(403)]
fn forbidden(req: &Request) -> String {
    format!("Forbidden access '{}'", req.uri())
}

#[catch(401)]
fn unauthorized(req: &Request) -> String {
    format!("Unauthorized access '{}'", req.uri())
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("Unknown error: {} ({})", status, req.uri())
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MoMConfig {
    host: String,
    port: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    moms: Vec<MoMConfig>,
}

pub async fn build_server() -> Rocket<Build> {
    let rocket = rocket::build();
    let figment = rocket.figment();
    let mom_config: Config = figment.extract().expect("config");

    rocket
        .attach(Db::init())
        .mount("/", routes![status])
        .mount("/", endpoints())
        .register(
            "/",
            catchers![
                internal_error,
                forbidden,
                unauthorized,
                not_found,
                bad_request,
                default
            ],
        )
        .attach(AdHoc::on_ignite("Manager thread", |rocket| async move {
            let db = Db::fetch(&rocket).unwrap().clone();
            let mut conn = db.acquire().await.unwrap();
            let mut moms = HashMap::new();

            for c in mom_config.moms.iter() {
                let client = Client::connect(c.host.clone(), c.port).await;
                let is_up = client.is_some();
                let mom = RegisteredMoM {
                    connection: client,
                    host: c.host.clone(),
                    port: c.port,
                };

                moms.insert((c.host.clone(), c.port), mom);

                let mom_id = sqlx::types::uuid::Uuid::new_v4();
                crud::insert_mom(&mut conn, &mom_id, c.host.as_str(), c.port, is_up).await;
            }

            rocket.manage(AvailableMoMs::new(moms))
        }))
        .attach(AdHoc::on_ignite("Manager thread", |rocket| async {
            let db = Db::fetch(&rocket).unwrap().0.clone();
            task::spawn(async move {
                loop {
                    if let Ok(mut conn) = db.acquire().await {
                        time::sleep(time::Duration::from_secs(3)).await;
                        // TODO: Trigger mom management here
                        let moms = crud::select_all_moms(&mut conn).await;
                        println!("MoMs {:#?}", moms);
                    } else {
                        println!("Could not get DB lock");
                    }
                }
            });

            rocket
        }))
}

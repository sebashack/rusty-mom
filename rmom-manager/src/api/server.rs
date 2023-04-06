use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::tokio::task;
use rocket::{Build, Request, Rocket};
use rocket_db_pools::Database;

use super::endpoints::endpoints;
use crate::database::connection::Db;
use crate::database::crud;
use crate::manager::mom::{AvailableMoMs, RegisteredMoM};
use crate::manager::Manager;

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
            let db_pool = Db::fetch(&rocket).unwrap();
            let mut conn = db_pool.acquire().await.unwrap();
            let mut moms = Vec::new();

            for c in mom_config.moms.iter() {
                let mom = RegisteredMoM::new(c.host.clone(), c.port).await;
                let has_connection = mom.has_connection();

                moms.push(((c.host.clone(), c.port), mom));

                let mom_id = sqlx::types::uuid::Uuid::new_v4();
                crud::insert_mom(&mut conn, &mom_id, c.host.as_str(), c.port, has_connection).await;
            }

            let available_moms = AvailableMoMs::new(moms);
            let manager = Manager::new(available_moms.clone(), db_pool.0.clone());

            manager.restore_queues().await;
            task::spawn(async move { manager.run().await });

            rocket.manage(available_moms)
        }))
}

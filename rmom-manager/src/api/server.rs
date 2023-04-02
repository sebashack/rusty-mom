use futures::lock::Mutex;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::{Build, Request, Rocket};
use std::collections::HashMap;
use std::sync::Arc;

use super::endpoints::{endpoints, RegisteredMoM, RegisteredMoMs};
use crate::client::endpoints::Client;

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
    port: u16,
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

    let mut moms = HashMap::new();
    for c in mom_config.moms.iter() {
        let mom = RegisteredMoM {
            connection: Client::connect(c.host.clone(), c.port).await,
            host: c.host.clone(),
            port: c.port,
        };

        moms.insert((c.host.clone(), c.port), mom);
    }

    rocket
        .manage(RegisteredMoMs {
            moms: Arc::new(Mutex::new(moms)),
        })
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
}

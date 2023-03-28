use futures::lock::Mutex;
use rocket::http::Status;
use rocket::{Build, Request, Rocket};
use std::sync::Arc;

use super::endpoints::{endpoints, AvailableClients};
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

pub async fn build_server(host: String, port: u16) -> Rocket<Build> {
    let clients = Arc::new(Mutex::new(Client::connect(host, port).await));

    rocket::build()
        .manage(AvailableClients { clients })
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

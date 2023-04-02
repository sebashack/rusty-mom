#[macro_use]
extern crate rocket;

pub mod messages {
    tonic::include_proto!("messages");
}

pub mod api;
pub mod client;
pub mod database;

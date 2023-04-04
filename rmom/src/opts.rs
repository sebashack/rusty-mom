use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Opts {
    pub host: String,
    pub port: u16,
    pub queue_buffer_size: usize,
    pub message_ttl: i32,
    pub database: DbOpts,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbOpts {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub min_connections: u32,
    pub max_connections: u32,
}

pub fn read_opts_file(path: &str) -> Opts {
    let path = Path::new(path);
    let read_err = format!("Could not read file in '{:?}'", path);
    let contents = fs::read_to_string(path).expect(&read_err);

    parse_opts(contents.as_str())
}

// Helpers
fn parse_opts(input: &str) -> Opts {
    serde_yaml::from_str(input).unwrap()
}

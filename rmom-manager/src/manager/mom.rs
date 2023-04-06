use crate::client::endpoints::Client;
use futures::lock::{Mutex, MutexGuard};
use log::info;
use rand::prelude::IteratorRandom;
use rocket::serde::uuid::Uuid;
use rocket::tokio::time;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::sync::Arc;
use tonic::Code;

use crate::database::connection::PoolConnectionPtr;
use crate::database::crud::{self, MoMRecord};

type Host = String;
type Port = i32;
type Key = (Host, Port);

pub struct RegisteredMoM {
    pub connection: Option<Client>,
    pub host: String,
    pub port: i32,
}

pub enum ConnectionStatus {
    Available,
    Unavailable,
    Restablished,
}

impl RegisteredMoM {
    pub async fn probe_conn_status(
        &mut self,
        max_retries: u16,
        delay_millis: u64,
    ) -> ConnectionStatus {
        match self.connection {
            None => {
                if let Some(client) = Client::connect(self.host.clone(), self.port).await {
                    self.connection = Some(client);
                    info!("Restablished MoM at {}:{}", self.host, self.port);
                    ConnectionStatus::Restablished
                } else {
                    info!("Still Unvailable MoM at {}:{}", self.host, self.port);
                    ConnectionStatus::Unavailable
                }
            }
            Some(ref mut client) => {
                if RegisteredMoM::is_up_retry(client, max_retries, delay_millis).await {
                    ConnectionStatus::Available
                } else {
                    self.connection = None;
                    info!(
                        "Previously available MoM at {}:{} is now unavailable",
                        self.host, self.port
                    );
                    ConnectionStatus::Unavailable
                }
            }
        }
    }

    async fn is_up_retry(client: &mut Client, max_retries: u16, delay_millis: u64) -> bool {
        let mut i = 0;
        while i < max_retries {
            let response = client.get_heartbeat().await;
            match response {
                Ok(_) => return true,
                Err(code) => {
                    if code != Code::Unavailable {
                        return true;
                    }
                }
            }

            i += i;
            time::sleep(time::Duration::from_millis(delay_millis)).await;
        }

        false
    }
}

pub struct AvailableMoMs {
    moms: HashMap<Key, Arc<Mutex<RegisteredMoM>>>,
}

impl AvailableMoMs {
    pub fn new(moms: Vec<(Key, RegisteredMoM)>) -> Self {
        let mut locked_moms = HashMap::new();

        for (k, v) in moms {
            locked_moms.insert(k, Arc::new(Mutex::new(v)));
        }

        AvailableMoMs { moms: locked_moms }
    }

    pub fn get_value_iter(&self) -> Values<'_, (String, i32), Arc<Mutex<RegisteredMoM>>> {
        self.moms.values()
    }

    pub async fn acquire(&self, key: &Key) -> Option<MutexGuard<'_, RegisteredMoM>> {
        match self.moms.get(key) {
            None => None,
            Some(v) => Some(v.lock().await),
        }
    }

    pub async fn get_random_up_key(db: &mut PoolConnectionPtr) -> Option<(Key, Uuid)> {
        let moms = crud::select_all_moms(db).await;
        let random_mom: Option<&MoMRecord> = moms
            .iter()
            .filter(|m| m.is_up)
            .choose(&mut rand::thread_rng());

        random_mom.map(|mom| ((mom.host.clone(), mom.port), mom.id))
    }
}

impl Clone for AvailableMoMs {
    fn clone(&self) -> Self {
        let mut locked_moms = HashMap::new();

        for (k, v) in self.moms.iter() {
            locked_moms.insert(k.clone(), Arc::clone(v));
        }

        AvailableMoMs { moms: locked_moms }
    }
}

use crate::client::endpoints::Client;
use futures::lock::Mutex;
use rand::prelude::IteratorRandom;
use rocket::serde::uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

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

pub struct AvailableMoMs {
    pub moms: Arc<Mutex<HashMap<Key, RegisteredMoM>>>,
}

impl AvailableMoMs {
    pub fn new(moms: HashMap<Key, RegisteredMoM>) -> Self {
        AvailableMoMs {
            moms: Arc::new(Mutex::new(moms)),
        }
    }

    pub async fn get_random_up_key(db: &mut PoolConnectionPtr) -> Option<(Key, Uuid)> {
        let moms = crud::select_all_moms(db).await;
        let random_mom: Option<&MoMRecord> = moms
            .iter()
            .filter(|m| m.is_up)
            .choose(&mut rand::thread_rng());

        if let Some(mom) = random_mom {
            Some(((mom.host.clone(), mom.port), mom.id.clone()))
        } else {
            None
        }
    }
}

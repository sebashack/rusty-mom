use crate::client::endpoints::Client;
use futures::lock::Mutex;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use std::sync::Arc;

pub struct RegisteredMoM {
    pub connection: Option<Client>,
    pub host: String,
    pub port: i32,
}

pub struct RegisteredMoMs {
    pub moms: Arc<Mutex<HashMap<(String, i32), RegisteredMoM>>>,
}

impl RegisteredMoMs {
    pub fn new(moms: HashMap<(String, i32), RegisteredMoM>) -> Self {
        RegisteredMoMs {
            moms: Arc::new(Mutex::new(moms)),
        }
    }

    fn gandom_up(moms: &HashMap<(String, i32), RegisteredMoM>) -> Option<&RegisteredMoM> {
        unimplemented!()
    }

    fn get_random(moms: &HashMap<(String, i32), RegisteredMoM>) -> &RegisteredMoM {
        let keys: Vec<&(String, i32)> = moms.keys().collect();
        let key = keys.choose(&mut rand::thread_rng()).unwrap();
        moms.get(key).unwrap()
    }
}

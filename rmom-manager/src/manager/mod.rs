pub mod mom;

use log::warn;
use rocket::tokio::{task, time};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::database::crud;
use crate::manager::mom::{AvailableMoMs, ConnectionStatus};

pub struct Manager {
    moms: AvailableMoMs,
    db_pool: Pool<Postgres>,
    manager_cycle_secs: u64,
    life_probe_max_retries: u16,
    life_probe_retry_delay_millis: u64,
}

impl Manager {
    pub fn new(
        moms: AvailableMoMs,
        db_pool: Pool<Postgres>,
        manager_cycle_secs: u64,
        life_probe_max_retries: u16,
        life_probe_retry_delay_millis: u64,
    ) -> Self {
        Manager {
            moms,
            db_pool,
            life_probe_max_retries,
            life_probe_retry_delay_millis,
            manager_cycle_secs,
        }
    }

    pub async fn restore_queues(&self) {
        let db_pool = self.db_pool.clone();

        if let Ok(mut db_conn) = db_pool.acquire().await {
            let down_moms = crud::select_down_moms(&mut db_conn).await;

            for mom in down_moms {
                let queues = crud::select_queues_by_mom(&mut db_conn, &mom.host, mom.port).await;

                for queue in queues {
                    crud::delete_queue_channels(&mut db_conn, &queue.id).await;

                    if let Some((key, mom_id)) =
                        AvailableMoMs::get_less_loaded_and_up_key(&mut db_conn).await
                    {
                        let all_moms = self.moms.clone();
                        let mut available_mom_lock = all_moms.acquire(&key).await;

                        if let Some(v) = available_mom_lock.as_mut() {
                            if let Some(client) = v.get_client() {
                                match client.rebuild_queue(&queue.label.as_str()).await {
                                    Ok(_) => {
                                        crud::update_queue_mom(&mut db_conn, &queue.id, &mom_id)
                                            .await
                                    }
                                    Err(err) => {
                                        warn!("MoM replied with error on rebuilding queue: {err}")
                                    }
                                }
                            } else {
                                warn!("Failed to get MoM with connection");
                            }
                        } else {
                            warn!("Failed to get MoM lock");
                        }
                    } else {
                        warn!("Not available moms for restore")
                    }
                }
            }
        } else {
            warn!("Manager: failed to get DB connection");
        }
    }

    pub async fn run(&self) {
        loop {
            for mom in self.moms.get_value_iter() {
                let mom_ref = Arc::clone(mom);
                let db_pool = self.db_pool.clone();
                let all_moms = self.moms.clone();
                let max_retries = self.life_probe_max_retries;
                let retry_delay = self.life_probe_retry_delay_millis;

                task::spawn(async move {
                    let mut mom_lock = mom_ref.lock().await;
                    let host = mom_lock.host.clone();
                    let port = mom_lock.port;

                    match mom_lock.probe_conn_status(max_retries, retry_delay).await {
                        ConnectionStatus::Unavailable => {
                            if let Ok(mut db_conn) = db_pool.acquire().await {
                                crud::update_mom_is_up(&mut db_conn, host.as_ref(), port, false)
                                    .await;
                                let queue_records =
                                    crud::select_queues_by_mom(&mut db_conn, host.as_ref(), port)
                                        .await;
                                for q in queue_records {
                                    crud::delete_queue_channels(&mut db_conn, &q.id).await;

                                    if let Some((key, mom_id)) =
                                        AvailableMoMs::get_less_loaded_and_up_key(&mut db_conn)
                                            .await
                                    {
                                        let mut available_mom_lock = all_moms.acquire(&key).await;
                                        if let Some(v) = available_mom_lock.as_mut() {
                                            if let Some(client) = v.get_client() {
                                                match client
                                                    .rebuild_queue(q.label.as_str())
                                                    .await
                                                {
                                                    Ok(_) => crud::update_queue_mom(&mut db_conn, &q.id, &mom_id).await,
                                                    Err(err) => warn!("MoM replied with error on rebuilding queue: {err}")
                                                }
                                            } else {
                                                warn!("Failed to get MoM with connection");
                                            }
                                        } else {
                                            warn!("Failed to get MoM lock");
                                        }
                                    } else {
                                        warn!("There are no available MoMs. Reallocation of queue is not possible");
                                    }
                                }
                            } else {
                                warn!("Manager thread: failed to get DB connection");
                            }
                        }
                        ConnectionStatus::Restablished => {
                            if let Ok(mut db_conn) = db_pool.acquire().await {
                                crud::update_mom_is_up(&mut db_conn, host.as_ref(), port, true)
                                    .await;
                            } else {
                                warn!("Manager thread: failed to get DB connection");
                            }
                        }
                        ConnectionStatus::Available => {}
                    }
                    drop(db_pool);
                });
            }

            time::sleep(time::Duration::from_secs(self.manager_cycle_secs)).await;
        }
    }
}

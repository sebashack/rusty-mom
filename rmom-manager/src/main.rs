use rmom_manager::api::server::build_server;

#[rocket::main]
async fn main() {
    env_logger::init();

    let server = build_server().await;
    server.launch().await.unwrap();
}

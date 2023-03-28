use rmom_manager::api::server::build_server;

#[rocket::main]
async fn main() {
    env_logger::init();

    let server = build_server("127.0.0.1".to_string(), 50051).await;
    server.launch().await.unwrap();
}

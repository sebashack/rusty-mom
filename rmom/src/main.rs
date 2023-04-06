use rmom::broadcast::stream_server::StreamServer;
use rmom::opts::read_opts_file;

#[tokio::main]
async fn main() {
    env_logger::init();

    let maybe_path = std::env::args().nth(1);
    match maybe_path {
        Some(path) => {
            let opts = read_opts_file(path.as_str());
            let stream_server = StreamServer::new(
                opts.host,
                opts.port,
                //opts.external_host,
                //opts.external_port,
                opts.queue_buffer_size,
                opts.message_ttl,
                &opts.database,
            )
            .await;

            stream_server.restore_queues().await;
            stream_server.run().await;
        }
        None => println!(
            "Path to process file not provided. Usage: `rusty_proxy /path/to/process.yaml`"
        ),
    }
}

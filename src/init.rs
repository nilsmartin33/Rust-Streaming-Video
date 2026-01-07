use std::time::Duration;
use tracing::{error, info_span, Instrument};
use wtransport::{Endpoint, Identity, ServerConfig};
use wtransport::endpoint::endpoint_side::Server;

use crate::connection::handle_connection;

const SERVER_PORT: u16 = 4433;
const CERT_PATH: &str = "cert.pem";
const KEY_PATH: &str = "key.pem";
pub async fn server_config() -> anyhow::Result<ServerConfig> {
    let config = ServerConfig::builder()
        .with_bind_default(SERVER_PORT)
        .with_identity(Identity::load_pemfiles(CERT_PATH, KEY_PATH).await?)
        .keep_alive_interval(Some(Duration::from_secs(5)))
        .build();
    Ok(config)
}

pub async fn run_server_loop(server: Endpoint<Server>, video_path: &str) {
    for idx in 0.. {
        let incoming_session = server.accept().await;
        let video_path = video_path.to_string();

        tokio::spawn(handle_connection(incoming_session, video_path)
            .instrument(info_span!("Connection ", idx)));
    }
}

pub fn validate_video(path: &str) -> anyhow::Result<()> {
    if !std::path::Path::new(path).exists() {
        error!("File at path {path} does not exist !");
        std::process::exit(1);
    }
    Ok(())
}

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .init();
}
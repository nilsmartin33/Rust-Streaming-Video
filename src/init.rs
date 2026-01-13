use std::time::Duration;
use tracing::{error, info_span, Instrument};
use wtransport::{Endpoint, Identity, ServerConfig};
use wtransport::endpoint::endpoint_side::Server;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::connection::handle_connection;

pub async fn server_config_with_identity(identity: Identity) -> anyhow::Result<ServerConfig> {
    let config = ServerConfig::builder()
        .with_bind_default(0)
        .with_identity(identity)
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
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_env_filter(env_filter)
        .init();
}
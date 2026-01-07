mod init;
mod connection;

use anyhow::Result;
use tracing::{info};

const VIDEO_PATH: &str = "test.mp4";

use crate::init::{init_logging, validate_video, server_config, run_server_loop};


#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    info!("Finding Path video");

    validate_video(VIDEO_PATH)?;

    info!("Path video find !");

    info!("Starting configuration Server");

    let config = server_config().await?;
    let server = wtransport::Endpoint::server(config)?;

    info!("Server configuration is ready!");

    run_server_loop(server, VIDEO_PATH).await;

    Ok(())
}
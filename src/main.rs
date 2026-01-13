mod init;
mod connection;
mod html_content;

mod http_server;

use std::env;
use std::path::Path;
use anyhow::Result;
use tracing::{error, info, warn};

const VIDEO_PATH: &str = "test.mp4";

use wtransport::Identity;
use crate::init::{init_logging, validate_video, server_config_with_identity, run_server_loop};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    info!("Finding Path video");

    validate_video(VIDEO_PATH)?;

    info!("Path video find !");

    info!("Loading SSL certificates...");
    let identity = load_identity().await?;

    let cert_digest = identity.certificate_chain().as_slice()[0].hash();

    info!("Starting configuration Server");
    let config = server_config_with_identity(identity).await?;
    let server = wtransport::Endpoint::server(config)?;
    let wt_port = server.local_addr()?.port();

    info!("Server configuration is ready!");

    info!("Starting HTTP server...");
    let http_server = http_server::HttpServer::new(&cert_digest, wt_port).await?;

    info!("Servers ready!");
    info!("Open your browser at: http://127.0.0.1:{}", http_server.local_port());
    info!("WebTransport running on port: {}", wt_port);

    tokio::select! {
        result = http_server.serve() => {
            error!("HTTP server stopped: {:?}", result);
        }
        result = run_server_loop(server, VIDEO_PATH) => {
            error!("WebTransport server stopped: {:?}", result);
        }
    }

    Ok(())
}

async fn load_identity() -> Result<Identity> {
    if env::var("PRODUCTION").unwrap_or_default() == "true" {
        info!("Production mode: loading certificates from files");
        let cert_path = env::var("CERT_PATH").unwrap_or_else(|_| "cert.pem".to_string());
        let key_path = env::var("KEY_PATH").unwrap_or_else(|_| "key.pem".to_string());

        info!("   Certificate: {}", cert_path);
        info!("   Private key: {}", key_path);

        return Identity::load_pemfiles(&cert_path, &key_path).await
            .map_err(|e| anyhow::anyhow!("Failed to load certificates: {}", e));
    }

    if Path::new("cert.pem").exists() && Path::new("key.pem").exists() {
        info!("Found certificate files, attempting to load...");

        match Identity::load_pemfiles("cert.pem", "key.pem").await {
            Ok(identity) => {
                warn!("Using certificate files from disk");
                warn!("Note: Browser may not trust self-signed certificates");
                warn!("    For production, use Let's Encrypt or set PRODUCTION=true");
                warn!(" ️  For development, consider removing .pem files to use self-signed mode");
                return Ok(identity);
            }
            Err(e) => {
                warn!("  Failed to load certificate files: {}", e);
                warn!("  Falling back to self-signed certificate...");
            }
        }
    }

    info!("  Development mode: using self-signed certificate");
    info!("  This will work immediately in Chrome without certificate errors");
    Identity::self_signed(["localhost", "127.0.0.1", "::1"])
        .map_err(|e| anyhow::anyhow!("Failed to create self-signed certificate: {}", e))
}
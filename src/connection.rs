use tracing::{error, info};
use wtransport::Connection;
use wtransport::endpoint::IncomingSession;

pub async fn handle_connection(session: IncomingSession, video_path: String) {
    let result = handle_connection_impl(session, video_path).await;
    error!("{:?}", result);
}

async fn handle_connection_impl(session: IncomingSession, video_path: String) -> anyhow::Result<()> {
    let mut buffer = vec![0; 65536].into_boxed_slice();

    info!("Wait incoming request");

    let request = session.await?;
    info!(
        "New session: Authority: '{}', Path: '{}'",
        request.authority(),
        request.path()
    );

    let connection = request.accept().await?;
    info!("Waiting for data from client");

    loop {
        tokio::select! {
            stream = connection.accept_bi() => {
                let stream = stream?;
                handle_bi_stream(stream, &mut buffer, &connection, &video_path).await?;
            }
            stream = connection.accept_uni() => {
                let stream = stream?;
                handle_uni_stream(stream, &mut buffer, &connection).await?;
            }
            dgram = connection.receive_datagram() => {
                let dgram = dgram?;
                handle_datagram_stream(dgram, &connection).await?;
            }
        }
    }
}

async fn handle_bi_stream(
    mut stream: (wtransport::SendStream, wtransport::RecvStream),
    buffer: &mut [u8],
    connection: &Connection,
    video_path: &str
) -> anyhow::Result<()> {
    info!("Accepted BI Stream !");

    let Some(bytes) = stream.1.read(buffer).await? else {
        return Ok(());
    };

    let str_data = std::str::from_utf8(&buffer[..bytes])?;
    let command = str_data.trim();

    info!("Received command: '{}'", command);

    match command {
        "START_VIDEO" => {
            handle_start_video_command(stream.0, connection.clone(), video_path.to_string()).await?;
        }
        "GET_METADATA" => {
            handle_get_metadata_command(stream.0, video_path).await?;
        }
        _ => {
            handle_unknown_command(stream.0, command).await?;
        }
    }
    Ok(())
}

async fn handle_start_video_command(
    mut stream: wtransport::SendStream,
    connection: Connection,
    video_path: String
) -> anyhow::Result<()> {
    info!("Start video command");
    info!("   Video path: {}", video_path);
    info!("   Connection remote: {}", connection.remote_address());

    info!("   [TODO] Would start streaming video here...");

    stream.write_all(b"STARTING_VIDEO").await?;
    stream.finish().await?;
    Ok(())
}

async fn handle_get_metadata_command(
    mut stream: wtransport::SendStream,
    video_path: &str
) -> anyhow::Result<()> {
    info!("Get metadata command");
    info!("📊 GET_METADATA command received");
    info!("   Video path: {}", video_path);

    info!("   [TODO] Would read video metadata here...");

    let fake_metadata = r#"{"file":"test_video.mp4","size":1048576}"#;

    stream.write_all(fake_metadata.as_bytes()).await?;
    stream.finish().await?;

    info!("✅ Metadata sent to client");
    Ok(())
}

async fn handle_unknown_command(
    mut stream: wtransport::SendStream,
    command: &str) -> anyhow::Result<()> {
    info!("Unknown command received: '{}'", command);
    info!("Sending ACK...");

    stream.write_all(b"ACK").await?;
    stream.finish().await?;

    info!("ACK sent");
    Ok(())
}

async fn handle_uni_stream(
    mut stream: wtransport::RecvStream,
    buffer: &mut [u8],
    connection: &Connection,
) -> anyhow::Result<()> {
    info!("Accepted UNI stream");

    let Some(bytes_read) = stream.read(buffer).await? else {
        return Ok(());
    };

    let str_data = std::str::from_utf8(&buffer[..bytes_read])?;
    info!("Received (uni) '{}'", str_data.trim());

    let mut stream = connection.open_uni().await?.await?;
    stream.write_all(b"ACK_UNI").await?;
    stream.finish().await?;

    info!("UNI ACK sent");

    Ok(())
}

async fn handle_datagram_stream(
    dgram: wtransport::datagram::Datagram,
    connection: &Connection,
) -> anyhow::Result<()> {
    let str_data = std::str::from_utf8(&dgram)?;

    info!("Received (dgram) '{str_data}' from client");

    connection.send_datagram(b"ACK_DGRAM")?;

    info!("Datagram ACK sent");
    Ok(())
}

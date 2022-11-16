use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;
use std::error::Error;
use url::Url;

/// A thin wrapper around [`tokio_tungstenite::connect_async`].
pub async fn connect(
    addr: &str,
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn Error>> {
    let url = Url::parse(addr)?;

    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
    println!("WebSocket handshake has been successfully completed");
    Ok(ws_stream)
}

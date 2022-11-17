use std::error::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use url::Url;
pub type ShowdownStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// A thin wrapper around [`tokio_tungstenite::connect_async`].
pub async fn connect(addr: &str) -> Result<ShowdownStream, Box<dyn Error>> {
    let url = Url::parse(addr)?;

    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
    println!("WebSocket handshake has been successfully completed");
    Ok(ws_stream)
}

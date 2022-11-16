use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let connect_addr = "ws://sim.smogon.com/showdown/websocket".to_string();

    let ws_stream = haxorust::socket::connect(&connect_addr).await.unwrap();
    tokio::pin!(ws_stream);
    while let Some(msg) = ws_stream.next().await {
        println!("{:?}", msg);
    }
}

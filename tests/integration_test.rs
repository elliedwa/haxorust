use haxorust::login::login;
use tokio_stream::StreamExt;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::test]
async fn test_login() {
    let connect_addr = "ws://sim.smogon.com/showdown/websocket".to_string();

    let mut ws_stream = haxorust::socket::connect(&connect_addr)
        .await
        .expect("couldn't connect");
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        while let Ok(Message::Text(msg)) =
            ws_stream.next().await.expect("error reading stream")
        {
            use haxorust::protocol::Message::*;
            if let (_, ChallStr(challstr)) = haxorust::protocol::parse(&msg)
                .expect("couldn't parse challstr")
            {
                let assertion = login("Deceiving9908", dbg!(&challstr))
                    .await
                    .expect("problem logging in");
                tx.send(assertion)
                    .await
                    .expect("couldn't send assertion through channel");
            }
        }
    });

    if let Some(assertion) = rx.recv().await {
        println!("got = {:#?}", assertion);
        std::process::exit(0);
    }
}

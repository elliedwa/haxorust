use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::fmt;
use tokio_stream::StreamExt;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::socket::ShowdownStream;
use crate::{protocol, Result};

#[derive(Serialize, Debug)]
struct LoginParams<'a> {
    act: &'a str,
    name: &'a str,
    pass: &'a str,
    challstr: &'a str,
}

#[derive(Serialize, Debug)]
struct GetAssertionParams<'a> {
    act: &'a str,
    name: &'a str,
    challstr: &'a str,
}

#[derive(Debug, Clone)]
struct LoginError(String);

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for LoginError {}

/// Constructor for client login command to be sent to a PS server.
pub async fn login(name: &str, pass: &str, challstr: &str) -> Result<String> {
    let client = Client::new();
    let params = LoginParams {
        act: "login",
        name,
        pass,
        challstr,
    };
    let resp_body: String = client
        .post("http://play.pokemonshowdown.com/action.php")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str::<Value>(
        resp_body.strip_prefix(']').expect("no prefix"),
    ) {
        Ok(v) => Ok(v["assertion"].to_string()),
        Err(_) => Err(LoginError(resp_body).into()),
    }
}

pub async fn get_assertion(name: &str, challstr: &str) -> Result<String> {
    let client = Client::new();
    let params = GetAssertionParams {
        act: "getassertion",
        name,
        challstr,
    };
    let resp_body: String = client
        .post("http://play.pokemonshowdown.com/action.php")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    Ok(resp_body)
}

pub async fn challstr(stream: &mut ShowdownStream) -> Result<String> {
    while let Ok(Message::Text(msg)) =
        stream.next().await.expect("error reading stream")
    {
        if let (_, protocol::Message::ChallStr(challstr)) =
            protocol::parse(&msg).expect("couldn't parse challstr")
        {
            return Ok(challstr);
        } else {
            continue;
        }
    }
    Err(LoginError("couldn't login".to_string()).into())
}

#[cfg(test)]
mod tests {
    use crate::socket;

    use super::*;

    #[tokio::test]
    async fn test_challstr() {
        let mut stream =
            socket::connect("ws://sim.smogon.com/showdown/websocket")
                .await
                .expect("Could not connect to socket");
        let challstr =
            challstr(&mut stream).await.expect("Could not get challstr");
        eprintln!("{challstr}");
    }

    #[tokio::test]
    async fn test_get_assertion() {
        let _assertion = get_assertion("test", "test").await.unwrap();
    }

    #[tokio::test]
    async fn test_login() {
        let _assertion = login("test", "test", "test").await.unwrap();
    }
}

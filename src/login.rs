use std::fmt;
use std::error::Error;

use reqwest::Client;
use serde_json::Value;
use serde::Serialize;

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
pub async fn login(
    name: &str,
    pass: &str,
    challstr: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let params = LoginParams { act: "login", name, pass, challstr };
    let resp_body: String = client
        .post("http://play.pokemonshowdown.com/action.php")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str::<Value>(resp_body
        .strip_prefix(']')
        .expect("no prefix"))
    {
        Ok(v) => Ok(v["assertion"].to_string()),
        Err(_) => Err(LoginError(resp_body).into()),
    }
}

pub async fn get_assertion(
    name: &str,
    challstr: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let params = GetAssertionParams {
        act: "getassertion",
        name,
        challstr
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_assertion() {
        let _assertion = get_assertion("test", "test").await.unwrap();
    }

    #[tokio::test]
    async fn test_login() {
        let _assertion = login("test", "test", "test").await.unwrap();
    }
}

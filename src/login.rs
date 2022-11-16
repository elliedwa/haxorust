use serde_json::Value;

/// Constructor for client login command to be sent to a PS server.
pub async fn login(
    name: &str,
    challstr: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let params = [
        ("act", "login"),
        ("name", name),
        ("pass", r#""""#),
        ("challstr", challstr),
    ];
    let resp_body: String = client
        .post("https://play.pokemonshowdown.com/action.php")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str::<Value>(dbg!(&resp_body
        .strip_prefix(']')
        .expect("no prefix")))
    {
        Ok(v) => Ok(v["assertion"].to_string()),
        Err(_) => Ok(resp_body.to_string()),
    }
}

use crate::socket::ShowdownStream;
use crate::{login, socket, Result};
use futures_util::SinkExt;
use std::env;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Debug)]
pub struct Player {
    name: String,
    socket: ShowdownStream,
}

impl Player {
    pub async fn login_with_env() -> Result<Self> {
        let mut connection: ShowdownStream =
            socket::connect(&env::var("SHOWDOWN_SOCKET").unwrap_or(
                "ws://sim.smogon.com/showdown/websocket".to_string(),
            ))
            .await?;
        let challstr = login::challstr(&mut connection).await?;
        let name_key = "PS_USERNAME";
        let pass_key = "PS_PASSWORD";
        let name = env::var(name_key)?;
        let assertion = match env::var(pass_key) {
            Ok(pass) if !pass.is_empty() => {
                login::login(&name, &pass, &challstr).await?
            }
            _ => login::get_assertion(&name, &challstr).await?,
        };
        let message = format!("|/trn {},0,{}", &name, &assertion);
        connection.send(Message::Text(message)).await?;
        Ok(Self {
            name,
            socket: connection,
        })
    }

    pub fn socket_mut(&mut self) -> &mut ShowdownStream {
        &mut self.socket
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use tokio_stream::StreamExt;

    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn test_login_without_name_env() {
        env::remove_var("PS_USERNAME");
        Player::login_with_env().await.unwrap();
    }

    #[tokio::test]
    async fn test_player_login() {
        env::set_var("PS_USERNAME", "Zendigo0285");
        env::remove_var("PS_PASSWORD");
        let mut player = Player::login_with_env().await.unwrap();
        println!("{:#?}", player.socket_mut().next().await.unwrap());
    }
}

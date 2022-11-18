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
        let addr = env::var("SHOWDOWN_SOCKET").unwrap_or_else(|_| {
            "ws://sim.smogon.com/showdown/websocket".to_string()
        });
        let name_key = "PS_USERNAME";
        let pass_key = "PS_PASSWORD";
        let name = env::var(name_key)?;
        match env::var(pass_key) {
            Ok(pass) if !pass.is_empty() => {
                Self::try_login_as_registered(&name, &pass, &addr).await
            }
            _ => Self::try_login_as_unregistered(&name, &addr).await,
        }
    }

    pub fn socket_mut(&mut self) -> &mut ShowdownStream {
        &mut self.socket
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub async fn try_login_as_unregistered(
        name: &str,
        addr: &str,
    ) -> Result<Self> {
        let mut connection: ShowdownStream = socket::connect(addr).await?;
        let challstr = login::challstr(&mut connection).await?;
        let assertion = login::get_assertion(name, &challstr).await?;
        let message = format!("|/trn {},0,{}", name, &assertion);
        connection.send(Message::Text(message)).await?;
        Ok(Self {
            name: name.to_string(),
            socket: connection,
        })
    }

    pub async fn try_login_as_registered(
        name: &str,
        pass: &str,
        addr: &str,
    ) -> Result<Self> {
        let mut connection: ShowdownStream = socket::connect(addr).await?;
        let challstr = login::challstr(&mut connection).await?;
        let assertion = login::login(name, pass, &challstr).await?;
        let message = format!("|/trn {},0,{}", name, &assertion);
        connection.send(Message::Text(message)).await?;
        Ok(Self {
            name: name.to_string(),
            socket: connection,
        })
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use tokio_stream::StreamExt;

    use super::*;

    #[tokio::test]
    async fn test_login_without_name_env() {
        env::remove_var("PS_USERNAME");
        assert!(Player::login_with_env().await.is_err());
    }

    #[tokio::test]
    async fn test_login_with_name_env() {
        env::set_var("PS_USERNAME", "test");
        assert!(Player::login_with_env().await.is_ok());
    }

    #[tokio::test]
    async fn test_player_login() -> Result<()> {
        let mut rng = thread_rng();
        let x: u32 = rng.gen_range(1000..10000);
        let name = format!("{}{}", eff_wordlist::large::random_word(), x);

        env::set_var("PS_USERNAME", name);
        env::remove_var("PS_PASSWORD");
        let mut player = Player::login_with_env().await.unwrap();
        while let Ok(res) = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            player.socket_mut().next(),
        )
        .await
        {
            println!("{:#?}", res.unwrap()?);
        }
        Ok(())
    }
}

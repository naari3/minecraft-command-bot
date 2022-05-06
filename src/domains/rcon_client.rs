use rcon::Connection;
use tokio::net::TcpStream;

use crate::{config::Config, error::Error};

pub struct RCONClient {
    connection: Connection<TcpStream>,
}

impl RCONClient {
    pub async fn new() -> Result<Self, Error> {
        let config = Config::get();
        let address = format!("{}:25575", config.rcon_host);
        Ok(Self {
            connection: <Connection<TcpStream>>::builder()
                .enable_minecraft_quirks(true)
                .connect(address, &config.rcon_password)
                .await?,
        })
    }

    pub async fn cmd(&mut self, cmd: &str) -> Result<String, Error> {
        Ok(self.connection.cmd(cmd).await?)
    }
}

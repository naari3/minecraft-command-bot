use async_minecraft_ping::{ConnectionConfig, StatusResponse};

use crate::{config::Config, error::Error};

pub struct PingClient {
    connection: ConnectionConfig,
}

impl PingClient {
    pub async fn new() -> Self {
        let config = Config::get();
        Self {
            connection: ConnectionConfig::build(config.server_address),
        }
    }

    pub async fn ping(self) -> Result<StatusResponse, Error> {
        let connection = self.connection.connect().await?;
        let connection = connection.status().await?;
        Ok(connection.status)
    }
}

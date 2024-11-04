use async_minecraft_ping::{ConnectionConfig, StatusResponse};

use crate::{config::Config, error::Error};

pub struct PingClient {
    connection: ConnectionConfig,
}

impl PingClient {
    pub async fn new() -> Self {
        let config = Config::get();
        let mut connection = ConnectionConfig::build(config.server_address);
        if let Some(port) = config.server_port {
            connection = connection.with_port(port);
        }
        Self {
            connection,
        }
    }

    pub async fn ping(self) -> Result<StatusResponse, Error> {
        let connection = self.connection.connect().await?;
        let connection = connection.status().await?;
        Ok(connection.status)
    }
}

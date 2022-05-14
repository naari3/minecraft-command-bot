use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub rcon_host: String,
    pub rcon_password: String,
    pub discord_bot_token: String,
    pub discord_bot_prefix: String,
    pub minecraft_log_path: Option<String>,
    pub minecraft_log_channel_id: Option<String>,
}

impl Config {
    pub fn get() -> Self {
        envy::from_env::<Config>().unwrap()
    }
}

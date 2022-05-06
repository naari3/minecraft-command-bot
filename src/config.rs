use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub rcon_host: String,
    pub rcon_password: String,
    pub discord_bot_token: String,
}

impl Config {
    pub fn get() -> Self {
        envy::from_env::<Config>().unwrap()
    }
}

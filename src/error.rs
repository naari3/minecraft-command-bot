#[derive(Debug, err_derive::Error)]
#[error(display = "An error occurred.")]
pub enum Error {
    #[error(display = "rcon related error: {}", _0)]
    Rcon(#[source] rcon::Error),
    #[error(display = "url related error: {}", _0)]
    Url(#[source] url::ParseError),
    #[error(display = "reqwest related error: {}", _0)]
    Reqwest(#[source] reqwest::Error),
    #[error(display = "ping server related error: {}", _0)]
    PingServer(#[source] async_minecraft_ping::ServerError),
    #[error(display = "serenity related error: {}", _0)]
    Serenity(#[source] serenity::Error),
}

#[derive(Debug, err_derive::Error)]
#[error(display = "An error occurred.")]
pub enum Error {
    #[error(display = "rcon related error: {}", _0)]
    Rcon(#[source] rcon::Error),
}

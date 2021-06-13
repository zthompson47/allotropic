pub type Result<T> = std::result::Result<T, Error>;

pub fn err<T>(message: &str) -> Result<T> {
    Err(Error::Internal(message.to_string()))
}

#[derive(Debug)]
pub enum Error {
    Internal(String),
    Reqwest(reqwest::Error),
    Rusqlite(rusqlite::Error),
    Serde(serde_json::error::Error),
    StdIo(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::Serde(error)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Rusqlite(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::StdIo(error)
    }
}

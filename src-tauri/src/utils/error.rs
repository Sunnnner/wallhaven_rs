
// create the error type that represents all errors possible in our program

pub type WallResult<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SelectorErrorKind(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    BoxError(#[from] Box<dyn std::error::Error>),
    #[error("Error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Error: {0}")]
    TomeSerError(#[from] toml::ser::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self::Io(std::io::Error::new(std::io::ErrorKind::Other, message))
    }
}
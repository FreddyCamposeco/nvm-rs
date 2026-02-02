use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{context}: {source}")]
    Context {
        context: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    StripPrefix(#[from] std::path::StripPrefixError),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error("Anyhow error: {0}")]
    Anyhow(String),

    #[error("{0}")]
    Message(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Anyhow(err.to_string())
    }
}

pub fn with_context<E>(context: &str, err: E) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    Error::Context {
        context: context.to_string(),
        source: Box::new(err),
    }
}

pub fn message<S: Into<String>>(msg: S) -> Error {
    Error::Message(msg.into())
}


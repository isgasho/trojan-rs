use std::fmt::{self, Display, Formatter};
use std::{error, io, result};

pub type Result<T> = result::Result<T, Error>;

pub trait IntoError {}

#[derive(Debug)]
pub enum Error {
    Boxed(Box<dyn error::Error + Send>),
    String(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            Self::Boxed(e) => e.fmt(f),
        }
    }
}

impl<E: 'static + error::Error + Send + IntoError> From<E> for Error {
    fn from(e: E) -> Self {
        Self::Boxed(Box::new(e))
    }
}

impl Into<Error> for String {
    fn into(self) -> Error {
        Error::String(self)
    }
}

impl Into<Error> for &str {
    fn into(self) -> Error {
        Error::String(self.to_owned())
    }
}

impl IntoError for io::Error {}
impl IntoError for native_tls::Error {}

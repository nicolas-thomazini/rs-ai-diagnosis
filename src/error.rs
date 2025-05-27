use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error(pub Box<dyn StdError + Send + Sync>);

pub type Result<T> = std::result::Result<T, Error>;

impl<E> From<E> for Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn from(e: E) -> Self {
        Error(e.into())
    }
}
pub use quizzard_derive::*;
use thiserror::Error;

type Result<V> = std::result::Result<V, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An IO error occurred")]
    Io(#[from] std::io::Error),
    #[error("Error: {0}")]
    Other(&'static str),
}

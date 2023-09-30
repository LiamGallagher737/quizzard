pub use quizzard_derive::*;
use thiserror::Error;

pub trait SelectEnum: Sized + 'static {
    const VARIANTS: &'static [Self];
    fn prompt(&self) -> &'static str;
    fn to_index(&self) -> usize;
    fn from_index(n: usize) -> Option<Self>;
type Result<V> = std::result::Result<V, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An IO error occurred")]
    Io(#[from] std::io::Error),
    #[error("Error: {0}")]
    Other(&'static str),
}

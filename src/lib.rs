pub use quizzard_derive::*;

pub trait SelectEnum: Sized + 'static {
    const VARIANTS: &'static [Self];
    fn prompt(&self) -> &'static str;
    fn to_index(&self) -> usize;
    fn from_index(n: usize) -> Option<Self>;
}

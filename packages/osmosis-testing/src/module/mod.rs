use crate::runner::Runner;

mod bank;
mod gamm;
mod wasm;

#[macro_use]
pub mod macros;

pub use bank::Bank;
pub use gamm::Gamm;
pub use wasm::Wasm;

pub trait Module<'a, R: Runner<'a>> {
    fn new(runner: &'a R) -> Self;
}

extern crate dotenv;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate rand;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod model;
pub mod provider;
pub mod random;

pub use model::*;

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
// TODO: remove on new release of clippy that ship: https://github.com/rust-lang/rust-clippy/pull/5535
#![allow(clippy::used_underscore_binding)]

#[macro_use]
extern crate lazy_static;
extern crate reqwest;

pub mod web_api;

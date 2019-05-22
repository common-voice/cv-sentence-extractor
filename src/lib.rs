#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

pub mod app;
mod loader;
pub mod errors;
mod extractor;
mod character_map;
mod standard_characters;

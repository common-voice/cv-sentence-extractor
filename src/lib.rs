#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

pub mod app;
mod character_map;
pub mod errors;
mod extractor;
mod loader;
mod standard_characters;

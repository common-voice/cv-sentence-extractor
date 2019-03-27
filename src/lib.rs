#![feature(proc_macro_hygiene)]

extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate glob;
extern crate regex;

pub mod app;
mod loader;
mod extractor;
mod character_map;

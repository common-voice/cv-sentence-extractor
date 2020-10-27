extern crate clap;
extern crate glob;
extern crate itertools;
extern crate punkt;
extern crate rand;
extern crate regex;
extern crate serde_json;
extern crate toml;

pub mod app;
mod extractor;
mod checker;
mod replacer;
mod rules;
mod config;
mod loaders;

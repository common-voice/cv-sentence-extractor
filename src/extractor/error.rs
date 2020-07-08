//! Error management of extractor
use failure_derive::*;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Options conflict: {}", 0)]
    OptionsConflic(String),
}

use std::ffi::OsString;

use clap::{App, Arg, ArgMatches, SubCommand};
use crate::loader::load_file_names;
use crate::loader::load;
use crate::extractor::SentenceExtractor;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args<'a, I, T>(itr: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("common-voice-yotp")
        .about("extract wiki dumps in simplified Chinese")
        .version(VERSION)
        .author("Florian Merz <flomerz@gmail.com>")
        .subcommand(
            SubCommand::with_name("extract")
                .about("tempalte stuff like helm template does")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .takes_value(true)
                        .number_of_values(1)
                        .help("input dir to glob"),
                ),
        ).get_matches_from(itr)
}

pub fn run<I, T>(itr: I) -> Result<Vec<String>, String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let all_matches = parse_args(itr);
    let matches = match all_matches.subcommand_matches("extract") {
        Some(m) => m,
        _ => return Err(String::from("did we forget the extract subcommand?")),
    };
        let file_names = load_file_names(
        &matches
            .value_of("dir")
            .unwrap_or_default()
    )?;
    for file_name in file_names {
        let texts = load(&file_name)?;
        for text in texts {
            for sentence in SentenceExtractor::new(&text) {
                println!("{}", sentence);
            }
        }
    }
    Ok(vec!())
}
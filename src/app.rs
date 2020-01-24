use std::ffi::OsString;

use crate::extractor::extract;
use crate::loader::load_file_names;
use clap::{App, Arg, ArgMatches, SubCommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args<'a, I, T>(itr: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("common-voice-yotp")
        .about("Common Voice Sentence Extraction Helper")
        .version(VERSION)
        .author("Florian Merz <flomerz@gmail.com>")
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extract sentences from Wikipedia dump extracts using WikiExtractor")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("lang")
                        .takes_value(true)
                        .number_of_values(1)
                        .help("language"),
                )
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .takes_value(true)
                        .number_of_values(1)
                        .help("path to WikiExtractor folder"),
                )
                .arg(
                    Arg::with_name("no_check")
                        .short("n")
                        .long("no_check")
                        .takes_value(false)
                        .help("output all the sentences without verification")
                ),
        )
        .get_matches_from(itr)
}

pub fn run<I, T>(itr: I) -> Result<(), String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let all_matches = parse_args(itr);
    let matches = match all_matches.subcommand_matches("extract") {
        Some(m) => m,
        _ => return Err(String::from("did we forget the extract subcommand?")),
    };
    let file_names = load_file_names(&matches.value_of("dir").unwrap_or_default())?;
    let language = &matches.value_of("language").unwrap_or_else(|| "english");

    extract(&file_names, language, matches.is_present("no_check"))
}

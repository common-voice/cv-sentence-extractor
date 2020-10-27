use std::ffi::OsString;

use crate::config::Config;
use crate::extractor::extract;
use crate::loader::{load_wikiextractor};
use clap::{App, Arg, ArgMatches, SubCommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args<'a, I, T>(itr: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("common_voice_sentence_collector")
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
                        .help("language as identified by ISO code - for example en, de, es"),
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
    start(all_matches)
}

fn start(all_matches: ArgMatches) -> Result<(), String> {
    // Wikipedia
    if let Some(matches) = all_matches.subcommand_matches("extract") {
        let config = Config {
            language: String::from(matches.value_of("language").unwrap_or("en")),
            no_check: matches.is_present("no_check"),
            directory: String::from(matches.value_of("dir").unwrap_or_default()),
            max_sentences_per_text: 3,
            file_prefix: String::from("wiki_"),
        };

        extract(config, load_wikiextractor)
    } else {
        Err(String::from("did we forget to add a subcommand?"))
    }
}

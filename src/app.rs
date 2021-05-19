use clap::{App, Arg, ArgMatches, SubCommand};
use std::ffi::OsString;

use crate::extractor::extract;
use crate::loaders::{File, Wikipedia};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args<'a, I, T>(itr: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let no_check_argument = Arg::with_name("no_check")
                                .short("n")
                                .long("no_check")
                                .takes_value(false)
                                .global(true)
                                .help("output all the sentences without verification");
    let language_argument = Arg::with_name("language")
                                .short("l")
                                .long("lang")
                                .takes_value(true)
                                .number_of_values(1)
                                .help("language as identified by ISO code - for example en, de, es");
    let directory_argument = Arg::with_name("dir")
                                .short("d")
                                .long("dir")
                                .takes_value(true)
                                .number_of_values(1)
                                .help("path to folder with files to process");

    App::new("common_voice_sentence_collector")
        .about("Common Voice Sentence Extraction Helper")
        .version(VERSION)
        .author("Florian Merz <flomerz@gmail.com>")
        .arg(no_check_argument)
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extract sentences from Wikipedia dump extracts using WikiExtractor")
                .arg(&language_argument)
                .arg(&directory_argument)
        )
        .subcommand(
            SubCommand::with_name("extract-file")
                .about("Extract sentences from files which have one sentence per line")
                .arg(&language_argument)
                .arg(&directory_argument)
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
    let no_check = all_matches.is_present("no_check");

    // Wikipedia
    if let Some(matches) = all_matches.subcommand_matches("extract") {
        let language = String::from(matches.value_of("language").unwrap_or("en"));
        let directory = String::from(matches.value_of("dir").unwrap_or_default());

        let wikipedia_loader = Wikipedia::new(language, directory);
        return extract(wikipedia_loader, no_check);
    }

    // File
    if let Some(matches) = all_matches.subcommand_matches("extract-file") {
        let language = String::from(matches.value_of("language").unwrap_or("en"));
        let directory = String::from(matches.value_of("dir").unwrap_or_default());

        let file_loader = File::new(language, directory);
        return extract(file_loader, no_check);
    }

    println!("{}", all_matches.usage());
    Err(String::from("Did you forget to add a subcommand?"))
}

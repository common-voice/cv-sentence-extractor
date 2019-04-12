use std::ffi::OsString;

use crate::extractor::choose;
use crate::languages::english::check;
use crate::loader::load;
use crate::loader::load_file_names;
use clap::{App, Arg, ArgMatches, SubCommand};
use punkt::TrainingData;
use rand::rngs::SmallRng;
use rand::FromEntropy;

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

    let mut char_count = 0;
    let mut sentence_count = 0;
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let rng = SmallRng::from_entropy();
            for sentence in choose(&text, &TrainingData::english(), rng, 3, check) {
                println!("{}", sentence);
                char_count += sentence.chars().count();
                sentence_count += 1;
            }
        }
        eprintln!("avg = {:?}", char_count as f64 / f64::from(sentence_count));
        eprintln!("count = {:?}", sentence_count);
    }
    Ok(())
}

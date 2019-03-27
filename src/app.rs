use std::ffi::OsString;

use crate::extractor::SentenceExtractor;
use crate::loader::load;
use crate::loader::load_file_names;
use clap::{App, Arg, ArgMatches, SubCommand};

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
    let file_names = load_file_names(&matches.value_of("dir").unwrap_or_default())?;

    let mut char_counts = vec![];
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let mut used_sentences = 0;
            let mut used_last = false;
            for sentence in SentenceExtractor::new(&text) {
                if used_sentences == 3 {
                    break;
                }

                if used_last {
                    used_last = false;
                    continue;
                }

                println!("{}", sentence);
                char_counts.push(sentence.chars().count());
                used_sentences += 1;
                used_last = true;
            }
        }
        eprintln!(
            "avg = {:?}",
            char_counts.iter().fold(0, |sum, n| sum + n) as f64 / char_counts.len() as f64
        );
        eprintln!("count = {:?}", char_counts.len());
    }
    Ok(vec![])
}

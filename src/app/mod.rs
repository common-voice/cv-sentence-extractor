use crate::character_map::SYMBOL_MAP;
use crate::errors::*;
use crate::extractor::SentenceExtractorBuilder;
use crate::loader::load;
use crate::loader::load_file_names;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::cmp::{max, min};
use std::ffi::OsString;

#[cfg(test)]
mod tests;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const AVG_CHAR_TIME: f64 = 0.25_f64;

#[rustfmt::skip]  // Skip rust fmt to place Args and Options with same format
pub fn parse_args<'a, I, T>(itr: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("common-voice-yotp")
        .about("extract wiki dumps in Chinese")
        .version(VERSION)
        .author("Florian Merz <flomerz@gmail.com>, Antonio Yang <yanganto@gmail.com>")
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
                )
                .arg(
                    Arg::with_name("trans")
                        .short("t")
                        .long("trans")
                        .help("automatically translate words from traditional Chinese into simplified Chinese"),
                )
                .arg(
                    Arg::with_name("short sentence length")
                        .short("s")
                        .long("short")
                        .takes_value(true)
                        .number_of_values(1)
                        .help("The suitable shortest sentence length"),
                )
                .arg(
                    Arg::with_name("long sentence length")
                        .short("l")
                        .long("long")
                        .takes_value(true)
                        .number_of_values(1)
                        .help("The suitable longest sentence length"),
                )
                .arg(
                    Arg::with_name("auxiliary symbols")
                        .short("a")
                        .long("aux")
                        .takes_value(true)
                        .number_of_values(1)
                        .help("The auxiliary symbols for extracting long sentence"),
                )
                .arg(
                    Arg::with_name("ignore symbols")
                        .short("i")
                        .long("ignore")
                        .takes_value(true)
                        .number_of_values(1)
                        .help("The symbols will be ignored when extracting"),
                ),
        )
        .get_matches_from(itr)
}

pub fn run<I, T>(itr: I) -> Result<Vec<String>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let all_matches = parse_args(itr);
    match all_matches.subcommand_name() {
        Some("extract") => extract(all_matches.subcommand_matches("extract").unwrap())?,
        _ => bail!("did we forget the extract subcommand?"),
    };

    Ok(vec![])
}

fn extract(matches: &ArgMatches) -> Result<()> {
    let file_names = load_file_names(matches.value_of("dir").unwrap())?;
    let shortest_length = matches
        .value_of("short sentence length")
        .unwrap_or("3")
        .parse::<usize>()
        .unwrap();
    let longest_length = matches
        .value_of("long sentence length")
        .unwrap_or("38")
        .parse::<usize>()
        .unwrap();
    let mut auxiliary_symbols: Vec<char> = matches
        .value_of("auxiliary symbols")
        .unwrap_or("")
        .chars()
        .map(|c| SYMBOL_MAP.get(&c).unwrap_or(&c).clone())
        .collect();
    let ignore_symbols: Vec<char> = matches
        .value_of("ignore symbols")
        .unwrap_or("")
        .chars()
        .map(|c| SYMBOL_MAP.get(&c).unwrap_or(&c).clone())
        .collect();

    let mut builder = SentenceExtractorBuilder::new()
        .translate(matches.is_present("trans"))
        .shortest_length(shortest_length)
        .longest_length(longest_length)
        .auxiliary_symbols(&mut auxiliary_symbols)?
        .ignore_symbols(&ignore_symbols)?;

    let mut sentences = vec![];

    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let mut article_sentences = vec![];
            for next in builder.build(&text) {
                article_sentences.push(next);
            }
            article_sentences.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap().reverse());
            let used_sentences = &mut article_sentences.clone()[..min(
                max(
                    (article_sentences.len() as f32 * 0.1_f32).floor() as usize,
                    3,
                ),
                article_sentences.len(),
            )]
                .to_owned();

            for sentence in used_sentences.clone() {
                println!("{}", sentence);
                sentences.push(sentence);
            }
        }

        eprintln!("count = {:?}", sentences.len());
        let characters = sentences
            .iter()
            .fold(0, |sum, sentence| sum + sentence.chars().count())
            as f64;
        let avg = characters / sentences.len() as f64;
        eprintln!("avg characters = {:?}", avg.floor());
        eprintln!(
            "hours = {:?}",
            (((characters * AVG_CHAR_TIME) as f64) / 60_f64 / 60_f64).floor() as i32,
        );
    }
    Ok(())
}

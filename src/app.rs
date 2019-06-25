use crate::errors::*;
use crate::extractor::SentenceExtractor;
use crate::loader::load;
use crate::loader::load_file_names;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::cmp::{max, min};
use std::ffi::OsString;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const AVG_CHAR_TIME: f64 = 0.25_f64;

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

    let mut word_vector_count = 0_i16;
    let mut sentences = vec![];
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let mut article_sentences = vec![];
            for next in SentenceExtractor::new(&text) {
                article_sentences.push(next);
            }
            article_sentences.sort_by(|a, b| a.sentence.len().partial_cmp(&b.sentence.len()).unwrap().reverse());
            let used_sentences = &mut article_sentences.clone()[..min(
                max(
                    (article_sentences.len() as f32 * 0.1_f32).floor() as usize,
                    3,
                ),
                article_sentences.len(),
            )]
                .to_owned();

            for next in used_sentences.clone() {
                println!("{}", next.sentence);
                if next.word_vectored {
                    word_vector_count += 1;
                }
                sentences.push(next.sentence);
            }

        }

        eprintln!("count = {:?}", sentences.len());
        eprintln!("word_vector_count = {:?}", word_vector_count);
        let characters = sentences
            .iter()
            .fold(0, |sum, sentence| sum + sentence.chars().count()) as f64;
        let avg = characters / sentences.len() as f64;
        eprintln!("avg characters = {:?}", avg.floor());
        eprintln!(
            "hours = {:?}",
            (((characters * AVG_CHAR_TIME) as f64) / 60_f64 / 60_f64).floor() as i32,
        );
    }
    Ok(())
}

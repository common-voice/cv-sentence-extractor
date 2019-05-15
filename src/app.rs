use std::cmp::{max, min};
use std::ffi::OsString;

use crate::extractor::SentenceExtractor;
use crate::loader::load;
use crate::loader::load_file_names;
use clap::{App, Arg, ArgMatches, SubCommand};

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

    let mut decile_char_counts: Vec<Vec<usize>> = (1..=5).map(|_| vec![]).collect();
    let mut article_sentences = vec![];
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let mut sentences = Vec::new();
            for sentence in SentenceExtractor::new(&text) {
                sentences.push(sentence);
            }

            let sentences_count = sentences.len();

            if sentences_count + 1 > article_sentences.len() {
                article_sentences.resize(sentences_count + 1, 0);
            }
            article_sentences[sentences_count] += 1;

            sentences.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap().reverse());

            for (i, elem) in decile_char_counts.iter_mut().enumerate() {
                let len = sentences_count;
                let count = max(
                    min(3, len),
                    ((len as f32) * ((i as f32) + 1_f32) * 5_f32 / 100_f32).floor() as usize,
                );
                elem.append(
                    &mut sentences[..count]
                        .iter()
                        .map(|s| s.chars().count())
                        .collect(),
                );
            }
        }

        eprintln!("article sentences = {:?}", article_sentences);
        for (i, elem) in decile_char_counts.iter().enumerate() {
            eprintln!("decile = {:?}%", (i + 1) * 5);
            eprintln!("count = {:?}", elem.len());
            let characters = elem.iter().fold(0, |sum, n| sum + n) as f64;
            let avg = characters / elem.len() as f64;
            eprintln!("avg characters = {:?}", avg.floor());
            eprintln!(
                "hours = {:?}",
                (((characters * AVG_CHAR_TIME) as f64) / 60_f64 / 60_f64).floor() as i32,
            );
            eprintln!("{}", "-".repeat(10));
        }
        eprintln!("{}", "-".repeat(10));
    }

    Ok(vec![])
}

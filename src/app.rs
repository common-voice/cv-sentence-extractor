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
                )
                .arg(Arg::with_name("trans").short("t").long("trans").help(
                    "automatically translate words from traditional Chinese into simplify Chinese",
                ))
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
                        .help("The suitable logest sentence length"),
                )
                .arg(
                    Arg::with_name("auxiliary symbols")
                        .short("a")
                        .long("aux")
                        .takes_value(true)
                        .number_of_values(1)
                        .help("The auxiliary symbols for extracting long sentence"),
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
    let auto_translate = matches.is_present("trans");
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
    let auxiliary_symbols: Vec<char> = matches
        .value_of("auxiliary symbols")
        .unwrap_or("")
        .chars()
        .collect();

    let mut sentences = vec![];
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let mut article_sentences = vec![];
            for next in SentenceExtractor::new_with_opt(
                &text,
                auto_translate,
                shortest_length,
                longest_length,
                auxiliary_symbols.clone(),
            ) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extractor() {
        let file_names = load_file_names("src/test_data/").unwrap();
        let texts = load(&file_names[0]).unwrap();
        let mut iter = SentenceExtractor::new_with_opt(
            texts[0].as_str(),
            false,
            3,
            38,
            vec!['，', '：', '；'],
        );
        assert_eq!(iter.next().unwrap(), "愛因斯坦係一位理論物理學家");
        assert_eq!(
            iter.next().unwrap(),
            "愛因斯坦喺德國烏爾姆市出世，一年後成家人搬咗去慕尼黑"
        );
        assert_eq!(iter.next().unwrap(), "佢屋企都係猶太人，但係冇入猶太教");
        assert_eq!(iter.next().unwrap(), "佢爸爸本來賣床褥，後來開電器舖");
        assert_eq!(
            iter.next().unwrap(),
            "五歲嗰年，佢爸爸送咗個指南針畀佢，佢就發現有啲睇唔到嘅嘢牽引住枝針"
        );
        assert_eq!(
            iter.next().unwrap(),
            "後來愛因斯坦話嗰次嘅經驗係佢一生中最有啟發性"
        );
        assert_eq!(
            iter.next().unwrap(),
            "雖然佢識砌啲機械模型嚟玩，但係讀書讀得好慢"
        );
        assert_eq!(
            iter.next().unwrap(),
            "可能係因為學習障礙病，又或者只係因為怕醜，又或者係因為佢個腦結構特殊"
        );
        assert_eq!(
            iter.next().unwrap(),
            "最新嘅理論話愛因斯坦應該係患咗亞氏保加症，係自閉症嘅一種"
        );
        assert_eq!(
            iter.next().unwrap(),
            "因為當時呢個病未畀人發現，佢父母重以為佢係低能"
        );
        assert_eq!(iter.next().unwrap(), "因為佢成功發現光電效應");
        assert_eq!(iter.next().unwrap(), "後來佢又寫咗好多有關時空，物質嘅理論");
        assert_eq!(
            iter.next().unwrap(),
            "不過，因為當時嘅人睇唔明佢嘅理論，導致佢無法得到應有嘅尊重"
        );
        assert_eq!(iter.next().unwrap(), "但至今，重有好多都睇唔明佢寫乜");
        assert_eq!(
            iter.next().unwrap(),
            "不過，最大唔同嘅係，人已經尊重佢，而唔係當佢癲佬"
        );
        assert!(iter.next().is_none());
    }
}

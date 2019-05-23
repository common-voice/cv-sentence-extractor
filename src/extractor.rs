use punkt::params::Standard;
use punkt::SentenceTokenizer;
use punkt::TrainingData;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::path::PathBuf;
use crate::loader::load;
use rand::rngs::SmallRng;
use rand::FromEntropy;
use crate::languages::english;
use crate::languages::french;

pub fn choose(
    text: &str,
    data: &TrainingData,
    mut rng: impl Rng,
    amount: usize,
    predicate: impl FnMut(&&str) -> bool,
) -> Vec<String> {
    SentenceTokenizer::<Standard>::new(text, data)
        .filter(predicate)
        .map(str::trim)
        .map(String::from)
        .choose_multiple(&mut rng, amount)
}

pub fn extract(file_names: &[PathBuf], language: &str) -> Result<(), String> {
    let (check, data) = match language {
        "english" => (english::check as fn(&&str) -> bool, TrainingData::english()),
        "french" => (french::check as fn(&&str) -> bool, TrainingData::french()),
        l => return Err(format!("unsupported language: {}", l)),
    };
    let mut char_count = 0;
    let mut sentence_count = 0;
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let rng = SmallRng::from_entropy();
            let mut sentences = choose(&text, &data, rng, 3, check);
            sentences.dedup();
            for sentence in sentences {
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
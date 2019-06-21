use crate::languages::checker;
use crate::loader::load;
use crate::config::load_config;
use punkt::params::Standard;
use punkt::SentenceTokenizer;
use punkt::TrainingData;
use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::FromEntropy;
use rand::Rng;
use std::path::PathBuf;

pub fn choose(
    language: &str,
    text: &str,
    data: &TrainingData,
    mut rng: impl Rng,
    amount: usize,
    predicate: impl FnMut(&&str) -> bool,
) -> Vec<String> {
    let config = load_config(&language);
    // FIXME: how do we pass the config to the check function?
    SentenceTokenizer::<Standard>::new(text, data)
        .filter(predicate)
        .map(str::trim)
        .map(String::from)
        .choose_multiple(&mut rng, amount)
}

pub fn extract(file_names: &[PathBuf], language: &str) -> Result<(), String> {
    let data = TrainingData::english(); // FIXME: how can we access this depending on data?
    let mut char_count = 0;
    let mut sentence_count = 0;
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let rng = SmallRng::from_entropy();
            let mut sentences = choose(&language, &text, &data, rng, 3, checker::check);
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

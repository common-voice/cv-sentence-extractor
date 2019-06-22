use crate::checker;
use crate::loader::load;
use crate::config::{load_config, Config};
use punkt::params::Standard;
use punkt::SentenceTokenizer;
use punkt::TrainingData;
use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::FromEntropy;
use rand::Rng;
use std::path::PathBuf;

pub fn choose(
    rules: &Config,
    text: &str,
    data: &TrainingData,
    mut rng: impl Rng,
    amount: usize,
    mut predicate: impl FnMut(&Config, &&str) -> bool,
) -> Vec<String> {
    SentenceTokenizer::<Standard>::new(text, data)
        .filter(|item| { predicate(rules, item) })
        .map(str::trim)
        .map(String::from)
        .choose_multiple(&mut rng, amount)
}

pub fn extract(file_names: &[PathBuf], language: &str) -> Result<(), String> {
    let config = load_config(&language);
    let data = get_training_data(language);
    let mut char_count = 0;
    let mut sentence_count = 0;
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let rng = SmallRng::from_entropy();
            let mut sentences = choose(&config, &text, &data, rng, 3, checker::check);
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

fn get_training_data(language: &str) -> TrainingData {
    let training_data = match language {
        "english" => TrainingData::english(),
        "czech" => TrainingData::czech(),
        "danish" => TrainingData::danish(),
        "dutch" => TrainingData::dutch(),
        "estonian" => TrainingData::estonian(),
        "finnish" => TrainingData::finnish(),
        "french" => TrainingData::french(),
        "german" => TrainingData::german(),
        "greek" => TrainingData::greek(),
        "italian" => TrainingData::italian(),
        "norwegian" => TrainingData::norwegian(),
        "polish" => TrainingData::polish(),
        "portuguese" => TrainingData::portuguese(),
        "slovene" => TrainingData::slovene(),
        "spanish" => TrainingData::spanish(),
        "swedish" => TrainingData::swedish(),
        "turkish" => TrainingData::turkish(),
        _ => TrainingData::english(),
    };
    training_data
}

use crate::replacer;
use crate::checker;
use crate::loader::load;
use crate::config::{load_config, Config};
use punkt::params::Standard;
use punkt::SentenceTokenizer;
use punkt::TrainingData;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::path::PathBuf;

const MAX_SENTENCES_PER_ARTICLE : usize = 3;

pub fn extract(file_names: &[PathBuf], language: &str, no_check: bool) -> Result<(), String> {
    let config = load_config(&language);
    let training_data = get_training_data(language);
    let mut char_count = 0;
    let mut sentence_count = 0;
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = load(&file_name)?;
        for text in texts {
            let sentences = choose(
                &config,
                &text,
                &training_data,
                MAX_SENTENCES_PER_ARTICLE,
                checker::check,
                replacer::replace_strings,
                no_check
            );

            for sentence in sentences {
                println!("{}", sentence);
                char_count += sentence.chars().count();
                sentence_count += 1;
            }
        }
        eprintln!("avg chars per sentence = {:?}", char_count as f64 / f64::from(sentence_count));
        eprintln!("count = {:?}", sentence_count);
    }
    Ok(())
}

fn choose(
    rules: &Config,
    text: &str,
    training_data: &TrainingData,
    amount: usize,
    predicate: impl FnMut(&Config, &str) -> bool,
    mut replacer: impl FnMut(&Config, &str) -> String,
    no_check: bool
) -> Vec<String> {
    let sentences_replaced_abbreviations: Vec<String> = SentenceTokenizer::<Standard>::new(text, training_data)
        .map(|item| { replacer(rules, item) })
        .collect();

    if no_check {
        sentences_replaced_abbreviations
    } else {
        pick_sentences(rules, sentences_replaced_abbreviations, amount, predicate)
    }
}

fn pick_sentences(
    rules: &Config,
    sentences_pool: Vec<String>,
    amount: usize,
    mut predicate: impl FnMut(&Config, &str) -> bool
) -> Vec<String> {
    let total_in_pool = sentences_pool.len();

    if total_in_pool < amount {
        return vec![];
    }

    if total_in_pool == 1 {
        return sentences_pool;
    }

    let mut iteration = 0;
    let mut chosen_sentences = vec![];
    let mut used_indexes = vec![];
    while chosen_sentences.len() < amount && iteration != total_in_pool - 1 {
        let rng = rand::thread_rng();
        let random_index: usize = get_not_yet_used_index(rng, total_in_pool - 1, &used_indexes);
        used_indexes.push(random_index);

        let sentence = &sentences_pool[random_index];
        if predicate(rules, &sentence) {
            chosen_sentences.push(sentence.trim().to_string());
            chosen_sentences.sort();
            chosen_sentences.dedup();
        }

        iteration = iteration + 1;
    }

    chosen_sentences
}

fn get_not_yet_used_index(mut rng: ThreadRng, max: usize, used_indexes: &Vec<usize>) -> usize {
    let mut index = rng.gen_range(0, max);
    let mut already_used = true;

    while already_used {
        index = rng.gen_range(0, max);
        already_used = used_indexes.contains(&index);
    }

    index
}

fn get_training_data(language: &str) -> TrainingData {
    match language {
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
    }
}

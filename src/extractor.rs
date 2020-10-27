use crate::config::Config;
use crate::replacer;
use crate::checker;
use crate::loader::load_file_names;
use crate::rules::{load_rules, Rules};
use punkt::params::Standard;
use punkt::{SentenceTokenizer, TrainingData};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::collections::HashSet;
use std::path::PathBuf;

pub fn extract(config:  Config, mut loader: impl FnMut(&PathBuf) -> Result<Vec<String>, String>) -> Result<(), String> {
    let rules = load_rules(&config.language);
    let training_data = get_training_data(&config.language);
    let mut existing_sentences = HashSet::new();
    let mut char_count = 0;
    let mut sentence_count = 0;
    let file_names = load_file_names(&config.directory, &config.file_prefix).unwrap();
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = loader(&file_name)?;
        for text in texts {
            let iteration_config = config.clone();
            let sentences = choose(
                &rules,
                &text,
                &existing_sentences,
                &training_data,
                &iteration_config,
                checker::check,
                replacer::replace_strings,
            );

            for sentence in sentences {
                println!("{}", sentence);
                char_count += sentence.chars().count();
                sentence_count += 1;
                existing_sentences.insert(sentence);
            }
        }
        eprintln!("avg chars per sentence = {:?}", char_count as f64 / f64::from(sentence_count));
        eprintln!("count = {:?}", sentence_count);
    }
    Ok(())
}

fn choose(
    rules: &Rules,
    text: &str,
    existing_sentences: &HashSet<String>,
    training_data: &TrainingData,
    config: &Config,
    predicate: impl FnMut(&Rules, &str) -> bool,
    mut replacer: impl FnMut(&Rules, &str) -> String,
) -> Vec<String> {
    let sentences_replaced_abbreviations: Vec<String> = SentenceTokenizer::<Standard>::new(text, training_data)
        .map(|item| { replacer(rules, item) })
        .collect();

    if config.no_check {
        sentences_replaced_abbreviations
    } else {
        pick_sentences(
            rules,
            sentences_replaced_abbreviations,
            existing_sentences,
            config.max_sentences_per_text,
            predicate,
        )
    }
}

fn pick_sentences(
    rules: &Rules,
    sentences_pool: Vec<String>,
    existing_sentences: &HashSet<String>,
    amount: usize,
    mut predicate: impl FnMut(&Rules, &str) -> bool
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
    let mut still_has_sentences_to_search = true;
    while chosen_sentences.len() < amount && still_has_sentences_to_search {
        let rng = rand::thread_rng();
        let random_index: usize = get_not_yet_used_index(rng, total_in_pool - 1, &used_indexes);
        used_indexes.push(random_index);

        let sentence = &sentences_pool[random_index];
        let not_already_chosen = !existing_sentences.contains(sentence);
        if predicate(rules, &sentence) && not_already_chosen {
            chosen_sentences.push(sentence.trim().to_string());
            chosen_sentences.sort();
            chosen_sentences.dedup();
        }

        iteration += 1;
        still_has_sentences_to_search = iteration < total_in_pool;
    }

    chosen_sentences
}

fn get_not_yet_used_index(mut rng: ThreadRng, max_index: usize, used_indexes: &[usize]) -> usize {
    let mut index = rng.gen_range(0, max_index + 1);
    let mut already_used = used_indexes.contains(&index);
    while already_used {
        index = rng.gen_range(0, max_index + 1);
        already_used = used_indexes.contains(&index);
    }

    index
}

fn get_training_data(language: &str) -> TrainingData {
    match language {
        "cs" => TrainingData::czech(),
        "de" => TrainingData::german(),
        "dk" => TrainingData::danish(),
        "el" => TrainingData::greek(),
        "en" => TrainingData::english(),
        "es" => TrainingData::spanish(),
        "et" => TrainingData::estonian(),
        "fi" => TrainingData::finnish(),
        "fr" => TrainingData::french(),
        "it" => TrainingData::italian(),
        "nl" => TrainingData::dutch(),
        "no" => TrainingData::norwegian(),
        "pl" => TrainingData::polish(),
        "pt" => TrainingData::portuguese(),
        "se" => TrainingData::swedish(),
        "sl" => TrainingData::slovene(),
        "tr" => TrainingData::turkish(),
        _ => TrainingData::english(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_true(_rules: &Rules, _sentence: &str) -> bool {
        true
    }

    fn check_false(_rules: &Rules, _sentence: &str) -> bool {
        false
    }

    #[test]
    fn test_get_not_yet_used_index() {
        let rng = rand::thread_rng();
        let max_index = 2;
        let used_indexes = vec![0, 2];

        assert_eq!(get_not_yet_used_index(rng, max_index, &used_indexes), 1);
    }

    #[test]
    fn test_pick_sentences_pool_smaller_than_amount() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![];
        let amount = 1;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 0);
    }

    #[test]
    fn test_pick_sentences_pool_one() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![String::from("Test")];
        let amount = 1;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true)[0], "Test");
    }

    #[test]
    fn test_pick_sentences_none_valid() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![
            String::from("Test"),
            String::from("Test2"),
            String::from("Test3"),
            String::from("Test4"),
        ];
        let amount = 3;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_false).len(), 0);
    }

    #[test]
    fn test_pick_sentences_only_pick_amount() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![
            String::from("Test"),
            String::from("Test2"),
            String::from("Test3"),
            String::from("Test4"),
        ];
        let amount = 3;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 3);
    }

    #[test]
    fn test_pick_sentences_no_dupes() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![
            String::from("Test"),
            String::from("Test"),
            String::from("Test"),
            String::from("Test"),
        ];
        let amount = 3;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 1);
    }

    #[test]
    fn test_pick_sentences_no_dupes_mixed() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![
            String::from("Test2"),
            String::from("Test"),
            String::from("Test2"),
            String::from("Test"),
        ];
        let amount = 3;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 2);
    }

    #[test]
    fn test_pick_sentences_no_existing_sentences() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let mut existing_sentences = HashSet::new();
        existing_sentences.insert(String::from("I am already existing"));
        existing_sentences.insert(String::from("I am already existing too"));
        let sentences = vec![
            String::from("I am already existing"),
            String::from("I am already existing too"),
        ];
        let amount = 2;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 0);
    }

    #[test]
    fn test_pick_sentences_no_existing_sentences_mixed() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let mut existing_sentences = HashSet::new();
        existing_sentences.insert(String::from("I am already existing"));
        existing_sentences.insert(String::from("Me too!"));
        let sentences = vec![
            String::from("Test"),
            String::from("I am already existing"),
            String::from("Test2"),
            String::from("Me too!"),
        ];
        let amount = 3;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 2);
    }

    #[test]
    fn test_pick_sentences_two_out_of_two() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![
            String::from("Test"),
            String::from("Test2"),
        ];
        let amount = 2;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 2);
    }
}

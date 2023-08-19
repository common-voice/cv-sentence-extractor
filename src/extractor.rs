use crate::config::Config;
use crate::replacer;
use crate::checker;
use crate::loaders::Loader;
use crate::rules::{load_rules, Rules};
use crate::segmenter::split_sentences_with_python;
use glob::glob;
use punkt::params::Standard;
use punkt::{SentenceTokenizer, TrainingData};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub fn extract(loader: impl Loader, no_check: bool, filter_list_path: String) -> Result<(), String> {
    let config = loader.get_config();
    let rules = load_rules(&config.language);
    let training_data = get_training_data(&config.language);
    let filtered_titles = read_filtered_titles(filter_list_path);
    let mut existing_sentences = HashSet::new();
    let mut char_count = 0;
    let mut sentence_count = 0;
    let file_names = load_file_names(&config.directory, &config.file_prefix).unwrap();
    for file_name in file_names {
        eprintln!("file_name = {:?}", file_name.to_string_lossy());
        let texts = loader.load(&file_name, &filtered_titles)?;
        for text in texts {
            let sentences = get_sentences(
                &rules,
                &text,
                &existing_sentences,
                &training_data,
                config,
                no_check,
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

fn get_sentences(
    rules: &Rules,
    text: &str,
    existing_sentences: &HashSet<String>,
    training_data: &TrainingData,
    config: &Config,
    no_check: bool,
) -> Vec<String> {
    let sentences_pool: Vec<String>;

    // We want to apply the replacements before we split into sentences, as otherwise
    // the segmentation would not take the replacement into account. This for example
    // would lead to rust-punkt splitting sentences wrongly, while with the replacement
    // it would have worked correctly.
    //
    // Example: Hi Mr. Smith, how are you?
    // might get split wrongly by the segmenter to "Hi Mr." and "Smith, how are you?"
    // If we apply a replacement for "Mr." before, this would for example end up as
    // "Hi Mister Smith, how are you?"
    let replaced_text = replacer::replace_strings(rules, text);

    if rules.segmenter != *"" {
        if rules.segmenter == "python" {
            sentences_pool = split_sentences_with_python(&config.language, &replaced_text);
        } else {
            panic!("Segmenter {} is not yet supported!", rules.segmenter);
        }
    } else {
        // we use rust-punkt as segmenter by default
        sentences_pool = SentenceTokenizer::<Standard>::new(&replaced_text, training_data)
            .map(|item| { String::from(item) })
            .collect();
    }

    if no_check {
        sentences_pool
    } else {
        pick_sentences(
            rules,
            sentences_pool,
            existing_sentences,
            config.max_sentences_per_text,
            checker::check,
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

    // We do not extract if the total is below the max amount.
    // This makes sure that we handle legal requirements correctly
    // such as not using the full corpus of a source.
    if total_in_pool <= amount && amount != usize::MAX {
        return vec![];
    }

    // If we're allowed to pick all sentences, we do not need to
    // select randomly
    if amount == usize::MAX {
        return sentences_pool.iter().filter(|&sentence| {
            let not_already_chosen = !existing_sentences.contains(sentence);
            predicate(rules, sentence) && not_already_chosen
        }).map(|sentence| {
            sentence.trim().to_string()
        }).collect::<Vec<_>>();
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
        if predicate(rules, sentence) && not_already_chosen {
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
    let mut index = rng.gen_range(0..=max_index);
    let mut already_used = used_indexes.contains(&index);
    while already_used {
        index = rng.gen_range(0..=max_index);
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

fn load_file_names(dir_name: &str, prefix: &str) -> Result<Vec<PathBuf>, String> {
    let chart_path = Path::new(dir_name);
    let glob_path = format!("{}/**/{}*", chart_path.to_string_lossy(), prefix);
    glob(&glob_path)
        .map_err(|e| format!("{}", e))?
        .map(|p| p.map_err(|e| format!("{}", e)))
        .collect::<Result<Vec<PathBuf>, String>>()
}

fn read_filtered_titles(filtered_titles_path: String) -> HashSet<String> {
    if filtered_titles_path.is_empty() {
        return HashSet::new();
    }

    eprintln!("Reading titles from {:?}", filtered_titles_path);
    let mut titles = HashSet::new();
    let titles_path = Path::new(&filtered_titles_path);
    let mut content = String::new();
    let mut file = File::open(titles_path).map_err(|e| format!("{}", e)).unwrap();
    file.read_to_string(&mut content)
        .map_err(|e| format!("{}", e)).unwrap();

    let all_titles = content.lines();
    for title in all_titles {
        titles.insert(title.to_string());
    }

    eprintln!("Read {:?} titles to filter for..", titles.len());
    titles
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
    fn test_pick_sentences_all_if_max_amount() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![
            String::from("Test"),
            String::from("Test2"),
            String::from("Test3"),
            String::from("Test4"),
            String::from("Test5"),
        ];
        let amount = usize::MAX;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 5);
    }

    #[test]
    fn test_pick_sentences_never_all_from_pool_if_not_max() {
        let rules : Rules = Rules {
            ..Default::default()
        };
        let existing_sentences = HashSet::new();
        let sentences = vec![String::from("Test")];
        let amount = 1;

        assert_eq!(pick_sentences(&rules, sentences, &existing_sentences, amount, check_true).len(), 0);
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
}

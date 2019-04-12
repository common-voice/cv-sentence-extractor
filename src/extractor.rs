use punkt::params::Standard;
use punkt::SentenceTokenizer;
use punkt::TrainingData;
use rand::seq::IteratorRandom;
use rand::Rng;

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

use std::iter::Iterator;
use std::iter::Peekable;
use std::vec::IntoIter;

static FULL_STOP: char = '。';

/// An Iterator extracting _sentences_ from a text node.
/// We throw away:
/// - Anything be fore the first [FULL_STOP]
/// - Anthing where [contains_no_invalid_char] returns `false`
/// - Anything after the last [FULL_STOP]
pub struct SentenceExtractor<'a> {
    sentences: Peekable<IntoIter<&'a str>>,
}

impl<'a> SentenceExtractor<'a> {
    pub fn new(sentences: &'a str) -> Self {
        SentenceExtractor {
            sentences: extract_sentences(sentences),
        }
    }
}

impl<'a> Iterator for SentenceExtractor<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.sentences.peek().is_some() {
            self.sentences.next()
        } else {
            None
        }
    }
}

fn contains_no_invalid_char(s: &str) -> bool {
    s.chars().all(char::is_alphabetic)
}

fn extract_sentences(value: &str) -> Peekable<IntoIter<&str>> {
    let v: Vec<&str> = value
        .trim_end_matches(|c| c != FULL_STOP)
        .split(FULL_STOP)
        .skip(1)
        .filter(|s| !s.is_empty())
        .filter(|s| contains_no_invalid_char(s))
        .collect(); // We should remove this collect and return the ugly type.
    v.into_iter().peekable()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_sentence_extractor() {
        let value = "唐王國。鹰潭号称铜都。高等";
        let mut iter = SentenceExtractor::new(value);
        assert_eq!(iter.next(), Some("鹰潭号称铜都"));
    }
}

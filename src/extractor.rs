use std::default::Default;

use regex::Regex;
use std::fmt::Debug;

use crate::character_map::CHARACTER_MAP;
use crate::standard_characters::STANDARD_CHARACTERS;

static TERMINAL_PUNCTUATIONS: [char; 4] = ['。', '？', '！', '\n'];
static PUNCTUATIONS: [char; 37] = [
    '"', '"', '、', '‧', '—', '—', '—', '～', '“', '”', '；', '·', '：', '‘', '•', '─', '兀', '∶',
    '∧', '∨', '，', '、', '．', '；', '：', '＃', '＆', '＊', '＋', '－', '＜', '＞', '＝', '＄',
    '％', '＠', '，',
];

#[derive(Debug)]
pub struct SentenceExtractor {
    text: String,
    /// Boolean option for translate words from traditional Chinese into simplify Chinese
    translate: bool,
    auxiliary_symbols: Vec<char>,
    shortest_length: usize,
    longest_length: usize,
    cut_with_auxiliary_symbols: bool,
    first_sentence: bool,
    previous_string: Option<String>,
}

impl Default for SentenceExtractor {
    fn default() -> Self {
        SentenceExtractor {
            text: String::new(),
            translate: true,
            auxiliary_symbols: vec!['，', '：', '；', '。', '？', '！', '\n'],
            shortest_length: 3,
            longest_length: 38,
            cut_with_auxiliary_symbols: true,
            first_sentence: true,
            previous_string: Some(String::new()),
        }
    }
}

impl SentenceExtractor {
    /// New the Extractor with translate option for automatically translate words from traditional Chinese into
    /// simplified Chinese
    pub fn new(
        text: &str,
        translate: bool,
        shortest_length: usize,
        longest_length: usize,
        auxiliary_symbols: Vec<char>,
    ) -> SentenceExtractor {
        let lines: Vec<&str> = text.lines().collect();
        let mut merge_symbols = auxiliary_symbols;
        merge_symbols.extend_from_slice(&TERMINAL_PUNCTUATIONS);
        SentenceExtractor {
            text: if lines.len() > 1 {
                // skip disambiguation pages
                if lines.first().unwrap().contains("消歧義") {
                    String::default()
                } else {
                    // skip title
                    lines[1..].join("")
                }
            } else {
                text.to_string()
            },
            translate,
            shortest_length,
            longest_length,
            auxiliary_symbols: merge_symbols,
            ..Default::default()
        }
    }
}

fn is_invalid(s: &str) -> bool {
    !s.chars().next().unwrap_or_default().is_alphabetic()
        || s.chars().any(|c| c.is_ascii())
        || s.chars().all(|c| !c.is_alphabetic())
}

lazy_static! {
    static ref PARANS: Regex = Regex::new("（.*）").unwrap();
}

impl Iterator for SentenceExtractor {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if self.text.len() == 0 {
                return None;
            }

            let chars = self.text.chars().collect::<Vec<_>>();
            let end_index = if self.cut_with_auxiliary_symbols {
                self.cut_with_auxiliary_symbols = false;
                chars
                    .iter()
                    .position(|&c| self.auxiliary_symbols.contains(&c))
            } else {
                chars
                    .iter()
                    .position(|&c| TERMINAL_PUNCTUATIONS.contains(&c))
            };
            let index = end_index.unwrap_or(chars.len());
            let mut next_item = if let Some(s) = self.previous_string.clone() {
                s
            } else {
                String::new()
            };
            next_item.push_str(
                &chars
                    .iter()
                    .take(index)
                    .collect::<String>()
                    .trim()
                    .to_string(),
            );
            self.text = chars
                .iter()
                .skip(index + (if end_index.is_some() { 1 } else { 0 }))
                .collect::<String>();

            // remove words in brackets
            next_item = PARANS.replace(&next_item, "").to_string();

            // transalte words into simplified format
            if self.translate {
                next_item = next_item
                    .chars()
                    .map(|c| CHARACTER_MAP.get(&c).unwrap_or(&c).clone())
                    .collect();
            }
            // adjust sentence in a suitable length
            let count = next_item.chars().count();
            if self.first_sentence && count < self.longest_length {
                self.previous_string = Some(next_item.to_string());
                self.cut_with_auxiliary_symbols = false;
                self.first_sentence = false;
                continue;
            } else if count >= self.longest_length && !self.first_sentence {
                self.cut_with_auxiliary_symbols = true;
                continue;
            } else if is_invalid(&next_item) || count < self.shortest_length {
                continue;
            } else if self.translate
                && next_item.chars().any(|c| {
                    !TERMINAL_PUNCTUATIONS.contains(&c)
                        && !PUNCTUATIONS.contains(&c)
                        && !STANDARD_CHARACTERS.contains(&c) // NOTE Standard characters only work for simplify chinese words
                })
            {
                continue;
            }

            self.previous_string = None;
            return Some(next_item.trim().to_string());
        }
    }
}

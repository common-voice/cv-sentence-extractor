use std::default::Default;

use regex::Regex;
use std::fmt::Debug;

use crate::character_map::{CHARACTER_MAP, SYMBOL_MAP};
use crate::standard_characters::STANDARD_CHARACTERS;

mod error;

type ExtractorError = error::Error;

static TERMINAL_PUNCTUATIONS: [char; 4] = ['。', '？', '！', '\n'];
static PUNCTUATIONS: [char; 37] = [
    '"', '"', '、', '‧', '—', '—', '—', '～', '“', '”', '；', '·', '：', '‘', '•', '─', '兀', '︰',
    '︿', '﹀', '，', '、', '．', '；', '：', '＃', '＆', '＊', '＋', '－', '＜', '＞', '＝', '＄',
    '％', '＠', '，',
];
static DEFAULT_AUXILIARY_SYMBOLS: [char; 7] = ['，', '：', '；', '。', '？', '！', '\n'];

#[derive(Debug, Clone)]
pub struct SentenceExtractor<'a> {
    text: String,
    /// Boolean option for translate words from traditional Chinese into simplify Chinese
    translate: bool,
    /// Symbols to cut sentence when it goes too long
    auxiliary_symbols: &'a [char],
    /// The Symbols will be ignored
    ignore_symbols: Option<&'a [char]>,
    /// Skip the sentence shorter than shortest length
    shortest_length: usize,
    /// Use auxiliary symbols to cut sentence when it longer than longest length
    longest_length: usize,
    /// Show the ending symbol of sentence if any
    ending_symbol: bool,
}

impl Default for SentenceExtractor<'_> {
    fn default() -> Self {
        SentenceExtractor {
            text: String::new(),
            translate: false,
            auxiliary_symbols: &DEFAULT_AUXILIARY_SYMBOLS,
            shortest_length: 3,
            longest_length: 38,
            ignore_symbols: None,
            ending_symbol: false,
        }
    }
}

pub struct SentenceExtractorBuilder<'a> {
    inner: SentenceExtractor<'a>,
}
impl<'a> SentenceExtractorBuilder<'a> {
    pub fn new() -> SentenceExtractorBuilder<'a> {
        SentenceExtractorBuilder {
            inner: SentenceExtractor::default(),
        }
    }
    pub fn build(&mut self, text: &str) -> SentenceExtractor {
        let lines: Vec<&str> = text.lines().collect();
        self.inner.text = if lines.len() > 1 {
            // skip disambiguation pages
            if lines.first().unwrap().contains("消歧義") {
                String::default()
            } else {
                // skip title
                lines[1..].join("")
            }
        } else {
            text.to_string()
        };
        self.inner.clone()
    }
    pub fn translate(mut self, translate: bool) -> Self {
        self.inner.translate = translate;
        self
    }
    pub fn shortest_length(mut self, shortest_length: usize) -> Self {
        self.inner.shortest_length = shortest_length;
        self
    }
    pub fn longest_length(mut self, longest_length: usize) -> Self {
        self.inner.longest_length = longest_length;
        self
    }
    pub fn chop_ending_symbol(mut self, chop: bool) -> Self {
        self.inner.ending_symbol = !chop;
        self
    }
    pub fn auxiliary_symbols(
        mut self,
        auxiliary_symbols: &'a mut Vec<char>,
    ) -> Result<Self, ExtractorError> {
        for s in auxiliary_symbols.iter() {
            if self.inner.ignore_symbols.unwrap_or_default().contains(s) {
                return Err(ExtractorError::OptionsConflic(format!(
                    "'{}' is ignored",
                    s
                )));
            }
        }
        auxiliary_symbols.extend_from_slice(&TERMINAL_PUNCTUATIONS);
        self.inner.auxiliary_symbols = auxiliary_symbols;
        Ok(self)
    }
    pub fn ignore_symbols(mut self, ignore_symbols: &'a Vec<char>) -> Result<Self, ExtractorError> {
        for s in ignore_symbols {
            if self.inner.auxiliary_symbols.contains(s) {
                return Err(ExtractorError::OptionsConflic(format!(
                    "'{}' is used to determine the cuting point for sentance",
                    s
                )));
            }
        }
        self.inner.ignore_symbols = Some(&ignore_symbols);
        Ok(self)
    }
}
// ignore_symbols: Vec<char>,

fn is_invalid(s: &str) -> bool {
    !s.chars().next().unwrap_or_default().is_alphabetic()
        || s.chars().any(|c| c.is_ascii())
        || s.chars().all(|c| !c.is_alphabetic())
}

lazy_static! {
    static ref PARANS: Regex = Regex::new("（.*）").unwrap();
}

impl<'a> SentenceExtractor<'a> {
    fn get_cutting_point<'b>(&self, chars: &'b Vec<char>) -> Option<(usize, Option<&'b char>)> {
        for (idx, c) in chars.iter().enumerate() {
            if (idx >= self.longest_length && self.auxiliary_symbols.contains(&c))
                || TERMINAL_PUNCTUATIONS.contains(&c)
            {
                if c.is_whitespace() {
                    return Some((idx, None));
                } else {
                    return Some((idx, Some(c)));
                }
            }
        }
        return None;
    }
}

impl<'a> Iterator for SentenceExtractor<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if self.text.len() == 0 {
                return None;
            }

            // normalized and disambiguate the input chars
            let chars = self
                .text
                .chars()
                .map(|c| SYMBOL_MAP.get(&c).unwrap_or(&c).clone())
                .filter(|c| {
                    if let Some(ignore_symbols) = self.ignore_symbols {
                        !ignore_symbols.contains(c)
                    } else {
                        true
                    }
                })
                .collect::<Vec<_>>();

            let end = self.get_cutting_point(&chars);
            let (index, ending_symbol) = end.unwrap_or((chars.len(), None));
            let mut next_item = chars
                .iter()
                .take(index)
                .collect::<String>()
                .trim()
                .to_string();
            self.text = chars
                .iter()
                .skip(index + (if end.is_some() { 1 } else { 0 }))
                .collect::<String>();

            // remove words in brackets
            next_item = PARANS.replace(&next_item, "").to_string();

            // translate words into simplified format
            if self.translate {
                next_item = next_item
                    .chars()
                    .map(|c| CHARACTER_MAP.get(&c).unwrap_or(&c).clone())
                    .collect();
            }
            let count = next_item.chars().count();
            if is_invalid(&next_item) || count < self.shortest_length {
                continue;
            } else if self.translate
                && next_item.chars().any(|c| {
                    !TERMINAL_PUNCTUATIONS.contains(&c)
                        && !PUNCTUATIONS.contains(&c)
                        && !STANDARD_CHARACTERS.contains(&c)
                    // NOTE Standard characters only work for simplify chinese words
                })
            {
                continue;
            }
            return if self.ending_symbol && ending_symbol.is_some() {
                Some(format!("{}{}", next_item.trim(), ending_symbol.unwrap()))
            } else {
                Some(next_item.trim().to_string())
            };
        }
    }
}

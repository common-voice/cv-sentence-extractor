use std::default::Default;

use regex::Regex;
use std::fmt::Debug;

use crate::character_map::{CHARACTER_MAP, SYMBOL_MAP};
use crate::standard_characters::STANDARD_CHARACTERS;

mod error;

static SHOWUP_PUNCTUATIONS: [char; 2] = ['？', '！'];
static TERMINAL_PUNCTUATIONS: [char; 10] =
    ['（', '）', '，', '。', '、', '：', '？', '；', '！', '\n'];
static PUNCTUATIONS: [char; 37] = [
    '"', '"', '、', '‧', '—', '—', '—', '～', '“', '”', '；', '·', '：', '‘', '•', '─', '兀', '︰',
    '︿', '﹀', '，', '、', '．', '；', '：', '＃', '＆', '＊', '＋', '－', '＜', '＞', '＝', '＄',
    '％', '＠', '，',
];

#[derive(Debug, Clone)]
pub struct SentenceExtractor<'a> {
    text: String,
    /// Boolean option for translate words from traditional Chinese into simplify Chinese
    translate: bool,
    /// Skip the sentence shorter than shortest length
    shortest_length: usize,
    /// Use auxiliary symbols to cut sentence when it longer than longest length
    longest_length: usize,
    /// The sentence including the black list symbols will be droped
    black_list_symbols: Option<&'a [char]>,
}

impl Default for SentenceExtractor<'_> {
    fn default() -> Self {
        SentenceExtractor {
            text: String::new(),
            translate: false,
            shortest_length: 3,
            longest_length: 38,
            black_list_symbols: None,
        }
    }
}
pub struct SentenceExtractorBuilder<'a> {
    inner: SentenceExtractor<'a>,
    /// The symbols will be ignored
    ignore_symbols: Option<&'a [char]>,
}
impl<'a> SentenceExtractorBuilder<'a> {
    pub fn new() -> SentenceExtractorBuilder<'a> {
        SentenceExtractorBuilder {
            inner: SentenceExtractor::default(),
            ignore_symbols: None,
        }
    }
    pub fn build(&mut self, text: &str) -> SentenceExtractor {
        let lines: Vec<&str> = text.lines().collect();

        self.inner.text = if lines.len() > 1 {
            // skip disambiguation pages
            if lines.first().unwrap().contains("消歧義") {
                String::default()
            } else {
                // skip title and normalized and disambiguate the input chars
                // ignore symbols
                lines[1..]
                    .join("")
                    .chars()
                    .map(|c| SYMBOL_MAP.get(&c).unwrap_or(&c).clone())
                    .filter(|c| {
                        if let Some(ignore_symbols) = self.ignore_symbols {
                            !ignore_symbols.contains(c)
                        } else {
                            true
                        }
                    })
                    .collect::<String>()
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
    pub fn ignore_symbols(mut self, ignore_symbols: &'a Vec<char>) -> Self {
        self.ignore_symbols = Some(&ignore_symbols);
        self
    }
    pub fn black_list_symbols(mut self, black_list_symbols: &'a Vec<char>) -> Self {
        self.inner.black_list_symbols = Some(&black_list_symbols);
        self
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
            if TERMINAL_PUNCTUATIONS.contains(&c) {
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

            let chars = self.text.chars().collect::<Vec<_>>();
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
            if let Some(black_list_symbols) = self.black_list_symbols {
                let mut has_black_symbols = false;
                for c in next_item.chars() {
                    if black_list_symbols.contains(&c) {
                        has_black_symbols = true;
                        break;
                    }
                }
                if has_black_symbols {
                    continue;
                }
            }

            let count = next_item.chars().count();
            if is_invalid(&next_item) || count < self.shortest_length || count > self.longest_length
            {
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
            if let Some(e) = ending_symbol {
                if SHOWUP_PUNCTUATIONS.contains(&e) {
                    return Some(format!("{}{}", next_item.trim(), e));
                }
            }
            return Some(next_item.trim().to_string());
        }
    }
}

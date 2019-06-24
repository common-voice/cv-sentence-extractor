use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex;

use regex::Regex;

use crate::character_map::CHARACTER_MAP;
use crate::errors::*;
use crate::standard_characters::STANDARD_CHARACTERS;

static PUNCTUATIONS: [char; 3] = ['。', '？', '！'];

pub struct SentenceExtractor {
    text: String,
}

pub struct NextSentence {
    pub sentence: String,
    pub word_vectored: bool
}

impl SentenceExtractor {
    pub fn new(text: &str) -> SentenceExtractor {
        let lines: Vec<&str> = text.lines().collect();
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
    static ref REPLACEMENTS: Mutex<HashMap<char, Result<Vec<char>>>> = Mutex::new(HashMap::new());
}

impl Iterator for SentenceExtractor {
    type Item = NextSentence;

    fn next(&mut self) -> Option<NextSentence> {
        loop {
            if self.text.len() == 0 {
                return None;
            }

            let chars = self.text.chars().collect::<Vec<_>>();
            let end_index = chars
                .iter()
                .position(|&c| PUNCTUATIONS.contains(&c) || c == '\n');
            let index = end_index.unwrap_or(chars.len());
            let mut next_item = chars
                .iter()
                .take(index)
                .collect::<String>()
                .trim()
                .to_string();
            self.text = chars
                .iter()
                .skip(index + (if end_index.is_some() { 1 } else { 0 }))
                .collect::<String>();

            next_item = PARANS.replace(&next_item, "").to_string();

            if is_invalid(&next_item) {
                continue;
            }

            next_item = next_item
                .chars()
                .map(|c| CHARACTER_MAP.get(&c).unwrap_or(&c).clone())
                .collect();

            let mut word_vectored = false;
            next_item = next_item
                .chars()
                .fold(Ok(vec![]), |vec, c| {
                    let mut vec = vec?.clone();
                    if PUNCTUATIONS.contains(&c) || STANDARD_CHARACTERS.contains(&c) || c == '，' {
                        vec.push(c);
                        return Ok(vec);
                    }

                    word_vectored = true;
                    let mut replacements = REPLACEMENTS.lock().unwrap();
                    let replacement = replacements.entry(c).or_insert_with(|| {
                        let output = Command::new("/bin/bash")
                            .arg("-c")
                            .arg(format!(
                                "~/fastText/fasttext nn ~/cc.zh.300.bin <<< '{}'",
                                c
                            ))
                            .output()
                            .unwrap();
                        let output = String::from_utf8(output.stdout).unwrap();

                        for line in output.split('\n') {
                            // skip the delimiting lines
                            if line.contains("Query word??") {
                                continue;
                            }

                            let parts = line.split(' ').collect::<Vec<&str>>();
                            let chars = parts[0];

                            if chars.chars().any(|c| {
                                !PUNCTUATIONS.contains(&c)
                                    || !STANDARD_CHARACTERS.contains(&c)
                                    || c != '，'
                            }) {
                                continue;
                            }

                            let perc = parts[1];
                            if perc.parse::<f32>()? > 0.8_f32 {
                                return Ok(chars.chars().collect::<Vec<char>>());
                            } else {
                                bail!("unlikely match");
                            }
                        }
                        bail!("no match found");
                    });
                    match replacement {
                        Ok(replacement) => {
                            vec.append(replacement);
                        }
                        _ => {
                            bail!("not replaceable");
                        }
                    }
                    Ok(vec)
                })
                .unwrap_or(vec![])
                .iter()
                .collect::<String>();

            let count = next_item.chars().count();
            if count < 3 || count > 38 {
                continue;
            }

            let abs_end = index + (if end_index.is_some() { 1 } else { 0 });
            self.text = chars
                .iter()
                .skip(
                    // skip over the next sentence, we don't want consecutive sentences
                    abs_end
                        + chars
                            .iter()
                            .skip(abs_end)
                            .position(|&c| PUNCTUATIONS.contains(&c) || c == '\n')
                            .unwrap_or(0),
                )
                .collect::<String>();

            if let Some(i) = end_index {
                next_item.push(*chars.get(i).unwrap());
            }
            return Some(NextSentence{sentence: next_item.trim().to_string(), word_vectored });
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_after() {
        let value = "唐王國。鹰号称铜。高等";
        let mut iter = SentenceExtractor::new(value);
        assert_eq!(iter.next().unwrap(), "鹰号称铜。");
        assert_eq!(iter.next().unwrap(), "高等");
        assert!(iter.next().is_none());
    }
}

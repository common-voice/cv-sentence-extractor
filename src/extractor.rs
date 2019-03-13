static PUNCTUATIONS: [char; 3] = ['。', '？', '！'];

use crate::traditional_characters::TRADITIONAL_CHARACTERS;

pub struct SentenceExtractor {
    text: String,
}

impl SentenceExtractor {
    pub fn new(text: &str) -> SentenceExtractor {
        SentenceExtractor {
            text: text.to_string(),
        }
    }
}

fn contains_invalid_char(s: &str) -> bool {
    s.chars()
        .any(|c| c.is_ascii() || !c.is_alphabetic() || TRADITIONAL_CHARACTERS.contains(&c))
}

impl Iterator for SentenceExtractor {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if self.text.len() == 0 {
                return None;
            }

            let chars = self.text.chars().collect::<Vec<_>>();
            let punctuation_index = chars.iter().position(|&c| PUNCTUATIONS.contains(&c));
            let index = punctuation_index.unwrap_or(chars.len());
            let mut next_item = chars.iter().take(index).collect::<String>();
            self.text = chars
                .iter()
                .skip(index + (if punctuation_index.is_some() { 1 } else { 0 }))
                .collect::<String>();

            if next_item.chars().count() <= 1 || contains_invalid_char(&next_item) {
                continue;
            }

            if let Some(i) = punctuation_index {
                next_item.push(*chars.get(i).unwrap());
            }
            return Some(next_item);
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

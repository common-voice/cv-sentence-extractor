use itertools::join;
use regex::Regex;

pub fn check(raw: &&str) -> bool {
    let trimmed = raw.trim();
    if trimmed.len() < 3
        || trimmed.chars().nth(0) == Some('"')
            && trimmed
                .chars()
                .nth(1)
                .map(|c| !c.is_alphabetic())
                .unwrap_or_default()
        || trimmed.ends_with(':')
    {
        return false;
    }
    let symbols = trimmed.chars().any(|c| {
        [
            '<', '>', '+', '*', '\\', '#', '@', '^', '[', ']', '(', ')', '/', '\n',
        ]
        .contains(&c)
    });
    if symbols {
        return false;
    }
    let broken_space = ["  ", " ,", " .", " ?", " !", " ;"];
    if broken_space.iter().any(|broken| raw.contains(broken)) {
        return false;
    }
    let words = trimmed.split_whitespace();
    let word_count = words.clone().count();
    let s = join(words, " ");
    if word_count == 0 || word_count > 14 {
        return false;
    }
    let numbers = s.contains(char::is_numeric);
    if numbers {
        return false;
    }
    let abrv = Regex::new(r"[[:upper:]]+\.*[[:upper:]]")
        .unwrap()
        .is_match(&s);
    if abrv {
        return false;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check() {
        assert_eq!(check(&"A.B"), false);
        assert_eq!(check(&r#""S.T.A.L.K.E.R."#), false);
        assert_eq!(check(&"foo\n\nfoo"), false);
    }
}

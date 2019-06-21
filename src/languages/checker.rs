use crate::config::Config;
use itertools::join;
use regex::Regex;

pub fn check(rules: Config, raw: &&str) -> bool {
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
    fn test_check_generic() {
        assert_eq!(check(&""), false);
        assert_eq!(check(&"\"Some test"), true);
        assert_eq!(check(&"\"😊"), false);
        assert_eq!(check(&"This ends with:"), false);
    }

    #[test]
    fn test_check_whitespace() {
        assert_eq!(check(&" AA "), false);
        assert_eq!(check(&"This has broken  space"), false);
        assert_eq!(check(&"This as well !"), false);
        assert_eq!(check(&"And this ;"), false);
    }

    #[test]
    fn test_check_length() {
        assert_eq!(check(&"This is gonna be way way way way way way way way way way too long"), false);
        assert_eq!(check(&"This is absolutely valid."), true);
    }

    #[test]
    fn test_check_numbers() {
        assert_eq!(check(&"This contains 1 number"), false);
    }

    #[test]
    fn test_check_symbols() {
        assert_eq!(check(&"foo\n\nfoo"), false);
        assert_eq!(check(&"foo<>"), false);
        assert_eq!(check(&"foo*@"), false);
    }

    #[test]
    fn test_check_abbreviations() {
        assert_eq!(check(&"A.B"), false);
        assert_eq!(check(&r#""S.T.A.L.K.E.R."#), false);
    }


    // FIXME: use FR config for following tests
    #[test]
    fn test_check_generic() {
        assert_eq!(check(&""), false);
        assert_eq!(check(&"\"😊"), false);
        assert_eq!(check(&"This ends with:"), false);
        assert_eq!(check(&"This does not end with a period"), false);
        assert_eq!(check(&"?This does not start with a alphanumeric letter"), false);
        assert_eq!(check(&"this starts with lowercase"), false);
    }

    #[test]
    fn test_check_whitespace() {
        assert_eq!(check(&" AA "), false);
        assert_eq!(check(&"This has broken  space"), false);
        assert_eq!(check(&"This as well !"), false);
        assert_eq!(check(&"And this ;"), false);
    }

    #[test]
    fn test_check_length() {
        assert_eq!(check(&"This is gonna be way way way way way way way way way way too long"), false);
        assert_eq!(check(&"Short"), false);
        assert_eq!(check(&"This is absolutely validé."), true);
        assert_eq!(check(&"No!!!"), false);
    }

    #[test]
    fn test_check_numbers() {
        assert_eq!(check(&"This contains 1 number"), false);
    }

    #[test]
    fn test_check_symbols() {
        assert_eq!(check(&"foo\n\nfoo"), false);
        assert_eq!(check(&"foo<>"), false);
        assert_eq!(check(&"foo«"), false);
        assert_eq!(check(&"foo*@"), false);
    }

    #[test]
    fn test_check_abbreviations() {
        assert_eq!(check(&"A.B"), false);
        assert_eq!(check(&"A."), false);
        assert_eq!(check(&"Some sentence that ends with A."), false);
        assert_eq!(check(&r#""S.T.A.L.K.E.R."#), false);
    }
}

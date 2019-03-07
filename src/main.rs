extern crate bzip2;
extern crate parse_mediawiki_dump;
extern crate parse_wiki_text;

use parse_wiki_text::{Configuration, Node};

static PUNCTUATIONS: [char; 3] = ['。', '？', '！'];
static INVALID_CHARS: [char; 3] = ['（', '）', '、'];

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("invalid use");
        std::process::exit(1);
    }
    let path = args.nth(1).unwrap();
    let file = match std::fs::File::open(&path) {
        Err(error) => {
            eprintln!("Failed to open input file: {}", error);
            std::process::exit(1);
        }
        Ok(file) => std::io::BufReader::new(file),
    };
    if path.ends_with(".bz2") {
        parse(std::io::BufReader::new(bzip2::bufread::BzDecoder::new(
            file,
        )));
    } else {
        parse(file);
    }
}

fn parse(source: impl std::io::BufRead) {
    let config = Configuration::default();
    for result in parse_mediawiki_dump::parse(source) {
        match result {
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
            Ok(page) => parse_text(&page.text, &config),
        }
    }
}

fn parse_text(text: &str, config: &Configuration) {
    let result = config.parse(text);
    if result.warnings.is_empty() {
        for node in result.nodes {
            if let Node::Text { value, .. } = node {
                extract_sentences(&value);
            }
        }
    }
}

fn extract_sentences(value: &str) {
    for sentence in value.split(&PUNCTUATIONS[..]) {
        let chars: Vec<char> = sentence.trim().chars().collect();
        if chars.len() == 0
            || chars[0] == '，'
            || chars
                .iter()
                .any(|c| c.is_numeric() || INVALID_CHARS.contains(c))
        {
            continue;
        }
        println!("{}", sentence);
    }
}

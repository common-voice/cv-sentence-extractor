extern crate bzip2;
extern crate parse_mediawiki_dump;
extern crate parse_wiki_text;

use parse_wiki_text::{Configuration, Node};

mod extractor;
mod traditional_characters;

use extractor::SentenceExtractor;

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
    if !result.warnings.is_empty() {
        return;
    }

    let mut text = String::new();
    for node in result.nodes {
        if let Node::Text { value, .. } = node {
            text.push_str(value);
        }
    }
    for s in SentenceExtractor::new(&text) {
        println!("{}", s)
    }
}

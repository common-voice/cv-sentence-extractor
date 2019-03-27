# Common Voice Wiki Sentence Extractor

## Dependencies

- Rust
- [WikiExtractor](https://github.com/attardi/wikiextractor)

## Usage

1. Use WikiExtractor to extract a dump.
```bash
cargo run -- extract -d <WIKI_EXTRACTOR_OUT_DIR>
```

## Rules
Currently it only works for the chinese wiki.
- extract at most 3 sentences per article
- skip disambiguation pages
- skip title
- skip sentences that...
    - start with non-alphabetic characters
    - contain ascii characters
    - contain only non-alphabetic characters
    - are shorter than 3 characters
    - are longer than 38 characters
- replaces traditional characters with their simplified counterpart

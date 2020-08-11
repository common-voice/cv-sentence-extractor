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
- randomly extract at most 3 sentences per article
- skip disambiguation pages
- skip title
- cutting sentence with these symbols `，`, `。`, `、`, `：`, `？`, `；`, `！`, `（` & `）`
- show the ending symbol if it is `？` or `！`
- skip sentences that...
  - start with non-alphabetic characters
  - contain ascii characters
  - contain only non-alphabetic characters
  - are shorter than 3 characters (can be changed with `-s` option)
  - are longer than 38 characters (can be changed with `-l` option)

## Options
- replaces traditional characters with their simplified counterpart (with `-t` option)
- ignore symbols (with `-i` option)
- load ignore symbols from a file (with `-I` option)
- black list symbols (with `-b` option)
- load black list symbols from a file (with `-B` option)
  - here is an example file, `/src/test_data/black_chars.txt`.

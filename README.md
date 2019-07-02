# Common Voice Wiki Sentence Extractor

## Dependencies

- [Rust Nightly](https://rustup.rs/) (follow the instructions and customize the install to select nightly channel)

We need to download the wikiextractor and this script
```
git clone https://github.com/attardi/wikiextractor.git
git clone https://github.com/Common-Voice/common-voice-wiki-scraper.git
```

## Usage

1. Download the latest wikipedia dataset [backup dump from Wikimedia](https://dumps.wikimedia.org/backup-index-bydb.html), select the one with `pages-articles-multistream` in its name.

Example (you can change "en" to your locale code)

```bash
wget https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-pages-articles-multistream.xml.bz2
tar -xzf enwiki-latest-pages-articles-multistream.xml.bz2
```

2. Use WikiExtractor to extract a dump

```bash
cd wikiextractor
python WikiExtractor.py --json ../enwiki-latest-pages-articles-multistream.xml
```

3. Scrap the sentences into a new file.
```bash
cd ../common-voice-wiki-scraper
cargo run -- extract -l english -d <WIKI_EXTRACTOR_OUT_DIR> >> wiki.en.txt
```

## Using language rules

We can only extract at most 3 sentences per article.

The following rules can be configured per language. Add a `<language>.tom` file in the `rules` directory to enable a new locale.

| Name   |      Description      |  Values | Default |
|--------|-----------------------|---------|---------|
| min_trimmed_length |  Minimum length of string after trimming | integer | 3
| min_alphanumeric_characters |  Minimum of alphanumeric character occurances | integer | 0
| may_end_with_colon |  If a sentence can end with a : or not | boolean | false
| quote_start_with_alphanumeric |  If a quote needs to start with an alphanumeric | boolean | true
| disallowed_symbols |  Array of disallowed symbols or letters | String Array | all symbols allowed
| broken_whitespace |  Array of broken whitespaces. This could for example disallow two spaces following eachother | String Array | all types of whitespaces allowed
| min_word_count |  Minimum number of words in a sentence | integer | 1
| max_word_count |  Maximum number of words in a sentence | integer | 14
| abbreviation_patterns |  Rust regex to match against | Rust Regex Array | all abbreviations allowed
| needs_punctuation_end |  If a sentence needs to end with a punctuation | boolean | false
| needs_alphanumeric_start |  If a sentence needs to start with an alphanumeric | boolean | true
| needs_uppercase_start |  If a sentence needs to start with an uppercase | boolean | false



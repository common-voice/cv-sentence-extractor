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
bzip2 -d enwiki-latest-pages-articles-multistream.xml.bz2
```

2. Use WikiExtractor to extract a dump (this might take a couple of hours)

```bash
cd wikiextractor
python WikiExtractor.py --json ../enwiki-latest-pages-articles-multistream.xml
```

3. Scrap the sentences into a new file from the WikiExtractor output dir (this might take more than 6h to finish)
```bash
cd ../common-voice-wiki-scraper
cargo run -- extract -l english -d ../wikiextractor/text/ >> wiki.en.txt
```

## Using language rules

We can only extract at most 3 sentences per article.

The following rules can be configured per language. Add a `<language>.toml` file in the `rules` directory to enable a new locale.

| Name   |      Description      |  Values | Default |
|--------|-----------------------|---------|---------|
| min_trimmed_length |  Minimum length of string after trimming | integer | 3
| min_characters |  Minimum of character occurances | integer | 0
| may_end_with_colon |  If a sentence can end with a : or not | boolean | false
| quote_start_with_letter |  If a quote needs to start with a letter | boolean | true
| disallowed_symbols |  Array of disallowed symbols or letters | String Array | all symbols allowed
| disallowed_words |  Array of disallowed words | String Array | all words allowed
| broken_whitespace |  Array of broken whitespaces. This could for example disallow two spaces following eachother | String Array | all types of whitespaces allowed
| min_word_count |  Minimum number of words in a sentence | integer | 1
| max_word_count |  Maximum number of words in a sentence | integer | 14
| abbreviation_patterns |  Rust regex to match against | Rust Regex Array | all abbreviations allowed
| needs_punctuation_end |  If a sentence needs to end with a punctuation | boolean | false
| needs_letter_start |  If a sentence needs to start with a letter | boolean | true
| needs_uppercase_start |  If a sentence needs to start with an uppercase | boolean | false

## Using disallowed words (blacklisting)

In order to increase the quality of the final output, you might want to consider filtering out some words that are complex, too long or non-native.

You can do this by adding these words to the language rules file for your language under the disallowed_words setting.

If your list of too long, you can also place a `<language>.txt` file in the `rules/disallowed_words` directory to enable a new locale. Each word should be in a new line.

### Create a blacklist based on less common words

You can create a solid blacklist by generating a list of the less common words from your wikipedia.

To do so, first you should create a full export with all wikipedia sentences. (Note that all processes below will take a while to execute)

After running step 1 and 2 from the Usage section above, run:

```bash
cd ../common-voice-wiki-scraper
cargo run -- extract -d ../wikiextractor/text/ --no_check >> wiki.en.all.txt
```

Then you can use the cvtools scripts to generate a list of the word frequency

```bash
cd  ..
git clone https://github.com/dabinat/cvtools/
cd cvtools
python3 ./word_usage.py -i ../common-voice-wiki-scraper/wiki.en.all.txt >> word_usage.en.txt
```

You will have to read the ``word_usage.en.txt`` file to decide where you should put the limit. Usually words with less than 80-60 repetitions are in general bad.

```bash
grep -i "80" ./word_usage.en.txt
```

Once you know the frequency limit, you can generate your blacklist by running

```bash
python3 ./word_usage.py -i ../common-voice-wiki-scraper/wiki.en.all.txt --max-frequency 80 --show-words-only >> ../common-voice-wiki-scraper/src/rules/disallowed_words/english.txt
```

This list will be automatically used if present when you run the scrapping on step 2 from the Usage section.

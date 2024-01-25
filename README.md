# Common Voice Sentence Extractor

[Common Voice](https://voice.mozilla.org) is Mozilla's initiative to help teach machines how real people speak. For this we need to collect sentences that people can read out aloud on the website. Individual sentences can be submitted through the [Sentence Collector](https://common-voice.github.io/sentence-collector). This only can scale so far, so we also use automated tools to extract sentences from other sources.

Right now this tool supports extractions from the following sources:

* Wikipedia - max 3 sentences per article
* Wikisource - max 3 sentences per article
* Simple files with one sentence per line

For a source to be added, the dataset needs to be vetted by Mozilla to check license compatibility. If you know about a good source, please start a topic on [Discourse](https://discourse.mozilla.org/c/voice/). Once it's been verified that a source can be used, check the "Adding another scrape target" further below.

## Flow

![Diagram](flow.svg)

*To edit this diagram, load the `flow.svg` in the root of the repository into [diagrams.net](https://app.diagrams.net/) and then save the updated version back into the repository like any other file changes you'd make.*

In the diagram above, light blue squares represent Sentence Extractor processes. The grey squares are processes outside of the Sentence Extractor tooling. The grey processes are the same for other sentence sources, such as bulk submissions and Sentence Extractor

1) If there is no rules file for the language you want to contribute to, then create a new rules file. The rules will be applied to every sentence picked from the datasource. If a sentence does not fulfill the rules, then it will be discarded and a new sentence will be chosen. The script randomly chooses sentences until the maximum allowed number of sentences is reached (if the data source has such a limit).
2) Check below on how to run the script. After running it, check the output and identify patterns in wrong sentences and add rules to capture these. The end goal is to have an error rate of less than 5%. You will most likely not get to 0% error rate, but aim for the lowest error rate reasonable. You can stop the script after a few seconds during this process to increase feedback time, as this process will require more than a single run. As sentences may be picked randomly, you will not see the same sentences when re-running the script. Therefore general patterns are important to be identified.
3) Once happy with the result, please create a PR with the rules file. Please also see the "Getting your rules/blocklist incorporated" section below for more information on what to include in this PR.
4) Once the PR is merged, a core contributor will run the extraction in the GitHub Actions pipeline. The resulting file will be used in the steps below.
5) The resulting output text file is added to the [language specific folder](https://github.com/common-voice/common-voice/tree/main/server/data) in the Common Voice repository through a PR. This is manual, but reflects the file from the previous step. No changes are done to this file to keep legal requirements fulfilled. Therefore this steps can only be performed by staff or a designated contributor.
6) Sentences added to the Common Voice `server/data` folder do not instantly get imported Common Voice. This means that they are not instantly available for recording on the Common Voice website. The import of new sentences only happens when a new version of the Common Voice website is released. You can find the past releases [here](https://github.com/common-voice/common-voice/releases).
7) If a certain language is enabled for contribution, the imported sentences will then be shown to contributors to record.

## Setup

- [Rust Nightly](https://rustup.rs/) (follow the instructions and customize the install to select the `nightly` channel)
- Install [`pip3`](https://pip.pypa.io/en/stable/installing/) in case it's not installed on your system already

Note: as long as we're using the current `inline-python` dependency, we need to use the Nightly version of Rust.

Clone this repo:

```
git clone https://github.com/Common-Voice/cv-sentence-extractor.git
```

### Wikipedia Extraction

You need to download the WikiExtractor:

```
git clone https://github.com/attardi/wikiextractor.git
```

## Extraction

### Extract Wikipedia

We can only extract at most 3 sentences per article.

1. Download the latest Wikipedia dataset [backup dump from Wikimedia](https://dumps.wikimedia.org/backup-index-bydb.html), select the one with `pages-articles-multistream` in its name.
If you don't want to download the whole dump file, you can also use the latest pages dump, e.g. https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-pages-articles1.xml-pXXXpXXX.bz2 (replace `XXX` with the numbering found in the directory).

Example (you can change "en" to your locale code)

```bash
wget https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-pages-articles-multistream.xml.bz2
bzip2 -d enwiki-latest-pages-articles-multistream.xml.bz2
```

2. Use WikiExtractor to extract a dump (this might take a few hours). In the parameters, we specify to use JSON as the output format instead of the default XML.

```bash
cd wikiextractor
git checkout e4abb4cbd019b0257824ee47c23dd163919b731b
python WikiExtractor.py --json ../enwiki-latest-pages-articles-multistream.xml
```

In order to test your setup or create a small test set, you can interrupt the extractor after a few seconds already, as it creates separate files in each step. Those files can be already ingested by the `cv-sentence-extractor`.
In the beginning, the WikiExtractor prints out how many processes it will use for the extraction: `INFO: Using 7 extract processes.` After that, a list of all extracted articles is printed out. As soon as some articles are extracted, you can abort the process and continue with step 3.

*Important note: Please check the section about [creating a rules file](#using-language-rules) and [a blocklist](#create-a-blocklist-based-on-less-common-words) at this point, you might want to consider creating them and that process should happen before step 3.*

3. Scrape the sentences into a new file from the WikiExtractor output dir (this might take more than 6h to finish)

```bash
cd ../cv-sentence-extractor
pip3 install -r requirements.txt # can be skipped if your language doesn't use the Python segmenter
cargo run --release -- -l en -d ../wikiextractor/text/ extract >> wiki.en.txt
```

*Tip: You don't need this last process to finish to start observing the output, wiki.en.txt should get a few thousands sentences in just a few minutes, and you can use that as a way to estimate the quality of the output early on and stop the process if you are not happy.*

#### Input file format
The input files ingested by `cv-sentence-extractor` are following a JSON format, based on what the WikiExtractor outputs. The format is simple:

    {
      "url": <url to wikipedia page via curid, string>,
      "text": <page text including title as first sentence with two blank lines, string>,
      "id": <page id, same as curid in url, string>,
      "title": <page title, string>
    }

Multiple pages in one file are separated without comma etc, just with a new line.

**Example**

    {
      "url": "https://de.wikipedia.org/wiki?curid=888306",
      "text": "Herzogtum Oels\n\nDas Herzogtum Oels...",
      "id": "888306",
      "title": "Herzogtum Oels"
    }

The file must follow the following directory structure and filename: `AA/wiki_XX`, with `AA` an alphanumerical numbering and `XX` two numbers for different files. The file should not have a file extension.

### Extract WikiSource

This process is very similar to the Wikipedia process above. We can only extract at most 3 sentences per article.

1. Download the latest Wikisource dataset [backup dump from Wikimedia](https://dumps.wikimedia.org/backup-index-bydb.html), select the one with `pages-articles` in its name.

Example (you can change "en" to your locale code)

```bash
wget https://dumps.wikimedia.org/enwikisource/latest//enwikisource-latest-pages-articles.xml.bz2
bzip2 -d enwikisource-latest-pages-articles.xml.bz2
```

2. Use WikiExtractor to extract a dump (this might take a few hours)

```bash
cd wikiextractor
git checkout e4abb4cbd019b0257824ee47c23dd163919b731b
python WikiExtractor.py --json ../enwikisource-latest-pages-articles.xml
```

*Important note: Please check the section about [creating a rules file](#using-language-rules) and [a blocklist](#create-a-blocklist-based-on-less-common-words) at this point, you might want to consider creating them and that process should happen before step 3.*

3. Scrape the sentences into a new file from the WikiExtractor output dir (this might take more than 6h to finish)

```bash
cd ../cv-sentence-extractor
pip3 install -r requirements.txt # can be skipped if your language doesn't use the Python segmenter
cargo run --release -- -l en -d ../wikiextractor/text/ extract-wikisource >> wiki.en.txt
```

*Tip: You don't need this last process to finish to start observing the output, wiki.en.txt should get a few thousands sentences in just a few minutes, and you can use that as a way to estimate the quality of the output early on and stop the process if you are not happy.*

### Extract from line break separated files

If you have one or multiple files with one sentence per line, you can use this extractor to extract sentences from these files applying the defined language rules. This can be useful if you have a large list of sentences and you want to only have sentences which match the rules.

```bash
pip3 install -r requirements.txt # can be skipped if your language doesn't use the Python segmenter
cargo run --release -- -l en -d ../texts/ extract-file >> file.en.txt
```

## Using language rules

The following rules can be configured per language. Add a `<language>.toml` file in the `rules` directory to enable a new locale. Note that the `replacements` get applied before any other rules are checked.

| Name   |      Description      |  Values | Default |
|--------|-----------------------|---------|---------|
| abbreviation_patterns |  Regex defining abbreviations | Rust Regex Array | all abbreviations allowed
| allowed_symbols_regex |  Regex of allowed symbols or letters. Each character gets matched against this pattern. | String Array | not used
| broken_whitespace |  Array of broken whitespaces. This could for example disallow two spaces following each other | String Array | all types of whitespaces allowed
| disallowed_symbols |  Use `allowed_symbols_regex` instead. Array of disallowed symbols or letters. Only used when allowed_symbols_regex is not set or is an empty String. | String Array | all symbols allowed
| disallowed_words |  Array of disallowed words. Prefer the blocklist approach when possible. | String Array | all words allowed
| even_symbols |  Symbols that always need an even count | Char Array | []
| matching_symbols |  Symbols that map to another | Array of matching configurations: each configuration is an Array of two values: `["match", "match"]`. See example below. | []
| max_word_count |  Maximum number of words in a sentence | integer | 14
| may_end_with_colon |  If a sentence can end with a : or not | boolean | false
| min_characters |  Minimum of character occurrences | integer | 0
| max_characters |  Maximum of character occurrences | integer | MAX
| min_trimmed_length |  Minimum length of string after trimming | integer | 3
| min_word_count |  Minimum number of words in a sentence | integer | 1
| needs_letter_start |  If a sentence needs to start with a letter | boolean | true
| needs_punctuation_end |  If a sentence needs to end with a punctuation | boolean | false
| needs_uppercase_start |  If a sentence needs to start with an uppercase | boolean | false
| other_patterns |  Regex to disallow anything else | Rust Regex Array | all other patterns allowed
| quote_start_with_letter |  If a quote needs to start with a letter | boolean | true
| remove_brackets_list |  Removes (possibly nested) user defined brackets and content inside them `(anything [else])` from the sentence before replacements and checking other rules | Array of matching brackets: each configuration is an Array of two values: `["opening_bracket", "closing_bracket"]`. See example below. | []
| replacements |  Replaces abbreviations or other words according to configuration. This happens before any other rules are checked. | Array of replacement configurations: each configuration is an Array of two values: `["search", "replacement"]`. See example below. | nothing gets replaced
| segmenter |  Segmenter to use for this language. See below for more information. | "python" | using `rust-punkt` by default
| stem_separator_regex |  If given, splits words at the given characters to reach the stem words to check them again against the blacklist, e.g. prevents "Rust's" to pass if "Rust" is in the blacklist. | Simple regex of separators, e.g. for apostrophe `stem_separator_regex = "[']"` | ""

### Example for `matching_symbols`

```
matching_symbols = [
  ["„", "“"],
  ["(", ")"],
  ["[", "]"]
]
```

This matches all occurrence of `„` with `“`, all occurrence of `(` with `)`, all occurrence of `[` with `]`.

```
Input: This is „a test“ and (another one)
Output: Valid

Input: This is (a test))
Output: Invalid
```

### Example for `remove_brackets_list`

```
remove_brackets_list = [
  ["(", ")"],
  ["[", "]"]
]
```

This internally creates related regex patterns and removes them with their content, executed in given order.

```
Input: This (parantheses) (and this) will be removed also this one (another [one]) should.
Output: This will be removed also this one should.

Input: This is (malformed)) at the source.
Output: This is ) at the source.
```

### Example for `replacements`

```
replacements = [
  ["test", "hi"],
  ["etc.", "et cetera"],
  ["foo", ""],
]
```

This replaces all occurrence of `test` with `hi`, all occurrence of `etc.` with `et cetera`, and removes all `foo`.

```
Input: I am a test etc.
Output: I am a hi et cetera

Input: I am foo test a test
Output: I am hi a hi
```

## Using disallowed words

In order to increase the quality of the final output, you might want to consider filtering out some words that are complex, too long or non-native.

You can do this by adding these words to the language rules file for your language under the `disallowed_words` setting.

If your list is too long, you can also place a `<language>.txt` file in the `rules/disallowed_words` directory to enable a new locale. Each word should be on a new line.

### Create a blocklist based on less common words

You can create a solid blocklist by generating a list of the less common words from your Wikipedia.

To do so, first you should create a full export with all Wikipedia sentences. Note that all processes below will take a while to execute.

After running step 1 and 2 from the `Usage` section above, run:

```bash
cd ../cv-sentence-extractor
cargo run --release -- -l en -d ../wikiextractor/text/ --no-check extract >> wiki.en.all.txt
```

Then you can use the cvtools scripts to generate a list of the word frequency:

```bash
cd  ..
git clone https://github.com/dabinat/cvtools/
cd cvtools
python3 ./word_usage.py -i ../cv-sentence-extractor/wiki.en.all.txt >> word_usage.en.txt
```

You will have to read the `word_usage.en.txt` file to decide where you should put the limit. Usually words with less than 80-60 repetitions are bad.

```bash
grep -i "80" ./word_usage.en.txt
```

Once you know the frequency limit, you can generate your blocklist by running:

```bash
python3 ./word_usage.py -i ../cv-sentence-extractor/wiki.en.all.txt --max-frequency 80 --show-words-only >> ../cv-sentence-extractor/src/rules/disallowed_words/en.txt
```

You can use also `--strip-by-apostrophe` which is handy for languages using `'` in their sentences to recognize more words.

When you run the scrapping in step 2 from the `Usage` section this list will automatically be used if present.

## Getting your rules/blocklist incorporated

In order to get your language rules and blocklist incorporated in this repo, you will need to create a Pull Request explaining the following:

- How many sentences did you get at the end?
- How did you create the blocklist file?
- Get at least 3 different native speakers (ideally linguistics) to review a random sample of 100-500 sentences and estimate the average error ratio and comment (or link their comment) in the PR. You can use [this template](https://docs.google.com/spreadsheets/d/1dJpysfcwmUwR4oJuw5ttGcUFYLeTbmn50Fpufz9qx-8/edit#gid=0) to simplify review.

Once we have your rules into the repo, we will run an automatic extraction and submit those sentences to Common Voice. This means that you can't manually adjust the sample output you've used for review as these changes would be lost.

## Using a different segmenter to split sentences

By default we are using the `rust-punkt` segmenter to split sentences. However this leads to several issues if `rust-punkt` does not support a given language. More info on that can be found in issue #11. Therefore we introduce a new way of adding your own Python-based segmenter if needed. Note that using Python-based segmenters will slow down the extract considerably.

If `rust-punkt` is not working well for a language rule file you are implementing, you can use your own custom segmenter written in Python. While English doesn't use a Python-based segmenter, there is an English example available in `src/segmenters.rs` you can use as base to write your own segmenter in Python.

This is currently experimental.

### Changes needed to add your own segmenter in Python

First you will need to add the `segmenter` rule to the rules file:

```
segmenter = "python"
```

This will direct our extraction script to use the special cases Python extraction.

Then you will need to add a new function to `src/segmenter.rs` with the name `split_sentences_with_python_xx`, replacing `xx` with your language code you also use for the rules file. You can copy/paste `split_sentences_with_python_en` and adjust it to your needs. Using Spanish as an example, your new function might look like this:

```
pub fn split_sentences_with_python_es(text: &str) -> Vec<String> {
    let ctx = Context::new();

    ctx.run(python! {
        import someLibraryWeNeed

        split_sentences = doTheNecessaryWorkToSplitSentences('text)
    });

    ctx.get("split_sentences")
}
```

Note that the function gets passed the full text as `text`, but you need to use `'text` to reference it within the Python block. This is a simple string with all sentences to be split. The split sentences need to be assigned to the `split_sentences` variable, as our script will read out this variable to continue the extraction.

Additionally you need to make sure that this function is called for your language, otherwise you will get an error that there is no matching function. For this, add a new match case to the `split_sentences_with_python` function. To add Spanish for example, add the following:

```
  "es" => split_sentences_with_python_es(text),
```

**Make sure you add all the required Python packages to `requirements.txt` as these will need to be installed by everyone running the respository locally as well as by the extraction pipelines on GitHub.**

As this is experimental, there are certain parts that could be improved, such as moving out each language into its own file, as well as automatically importing the needed file so there is no need to manually add a case to the match. PRs are certainly welcome!

## Adding another scrape target

If you find a new open data source that provides a lot of sentences ([Example](https://discourse.mozilla.org/t/using-the-europarl-dataset-with-sentences-from-speeches-from-the-european-parliament/50184/36)), we suggest to not go through through the Sentence Collector but rather adding a scrape target here. Before you do so, let's discuss it on [Discourse](https://discourse.mozilla.org/c/voice/) first!

* In `loaders` add your own loader file and write your own code according to the given data structure of your target - the data structure should be fairly simple, you might need to consider writing a separate script to fetch and prepare the sentences first (as we do with the WikiExtractor for Wikipedia). Note that you'll need to implement the `Loader` trait.
* In `loaders/mod.rs` expose your new file
* In `app.rs`, add a new extraction command - same arguments as the `extract` task, but with a better - more descriptive - name identifying your data source
* In `app.rs` add a new `if` in the `start` function to instantiate your extractor and start the extraction, passing your own custom extractor you wrote
* Add a new section in this README documenting the usage and purpose of your new target
* Add your new target to the list at the top of the README

You can find an example in the [File Loader Commit](https://github.com/Common-Voice/cv-sentence-extractor/commit/c0f3c81f021b7c7bc96bc01302af54422d69c193). Note that code might have slightly changed, but the concept is the same.

## Automatic extraction

Currently the following data sources are available for automatic extraction:

* Wikipedia

### On every Pull Request

On every PR we will [trigger a sample sentence extraction](https://discourse.mozilla.org/t/scraper-automatic-sample-sentences-extracted-in-pull-request/55217/3) which can be used for verification. Note that [GitHub does not automatically run](https://github.blog/2021-04-22-github-actions-update-helping-maintainers-combat-bad-actors/) the pipeline if you are a first time contributor. If your sample extraction doesn't get approved within a day, please reach out to us on [Matrix](https://matrix.to/#/#common-voice-sentence-extractor:mozilla.org?web-instance[element.io]=chat.mozilla.org).

## Manual trigger

### Through the manual workflow

Once a language rule file has been merged, the creation of the extract will be triggered through the manual workflow. PR authors do not need to do that themselves, this is the responsibility of the reviewer.

There are manual workflows for both Wikipedia and WikiSource.

### Through comments
Jobs can be triggered manually by adding a comment to an issue or Pull Request. Note that the blocklist uses the Wikipedia scrape target behind the scene.

```
/action [job] [language] [otherParams]
```

* job: name of the job to run, this can be any of: blocklist
* language: language code to process for: en, de, ...
* otherParams: any other params needed depending on the job

The job will then add a comment with its URL, so you can check the output and download the files you need.

*Example:* Create a blocklist for English - 80 occurrences threshold

```
/action blocklist en 80
```

### Re-running the extraction

There is a manual workflow trigger which allows us to re-run Wikipedia extractions on articles added since the last time we ran the extraction. This makes sure that we only extract new articles, as otherwise we would not fullfil the legal requirements. Any re-run needs to be triggered through this workflow.

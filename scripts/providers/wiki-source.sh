#!/bin/bash

WIKI_EXTRACTOR_URL="https://raw.githubusercontent.com/attardi/wikiextractor/e4abb4cbd019b0257824ee47c23dd163919b731b/WikiExtractor.py"
WIKI_EXTRACTOR_PATH="$WORKSPACE/WikiExtractor.py"

function run {
  echo "Getting WikiExtractor"
  curl $WIKI_EXTRACTOR_URL > $WIKI_EXTRACTOR_PATH

  echo "Starting extraction for $ARCHIVE_FILE_NAME"
  extract
  cleanup
}

function _downloadAndDecompressDump {
  FILE_NAME="${LANGUAGE_CODE}wikisource-latest-pages-articles.xml"
  ARCHIVE_FILE_NAME="${FILE_NAME}.bz2"
  DUMP_URL="https://dumps.wikimedia.org/${LANGUAGE_CODE}wikisource/latest/${ARCHIVE_FILE_NAME}"
  echo "Downloading dump for $LANGUAGE_CODE at $DUMP_URL"
  curl $DUMP_URL > $WORKSPACE/$ARCHIVE_FILE_NAME
  echo "Extracting dump - $ARCHIVE_FILE_NAME"
  bzip2 -d -k $WORKSPACE/$ARCHIVE_FILE_NAME
  DUMP_FILE="$WORKSPACE/$FILE_NAME"
}

function extract {
  _downloadAndDecompressDump

  echo "Extracting with WikiExtractor"
  python $WIKI_EXTRACTOR_PATH --processes 4 --json $DUMP_FILE

  echo "Running extraction"
  cargo run --release -- -l $LANGUAGE_CODE -d $EXTRACTED_TEXT_PATH extract >> $EXTRACTED_SENTENCES_PATH
}

function cleanup {
  rm -rf $WORKSPACE/$ARCHIVE_FILE_NAME
  rm -rf $DUMP_FILE
  rm -rf $EXTRACTED_TEXT_PATH
}

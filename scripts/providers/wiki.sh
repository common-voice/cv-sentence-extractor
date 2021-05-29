#!/bin/bash
set -x

WIKI_EXTRACTOR_URL="https://raw.githubusercontent.com/attardi/wikiextractor/e4abb4cbd019b0257824ee47c23dd163919b731b/WikiExtractor.py"
WIKI_EXTRACTOR_PATH="$WORKSPACE/WikiExtractor.py"

function setup {
  FILE_NAME="wiki-latest-pages-articles-multistream.xml"
  ARCHIVE_FILE_NAME="${FILE_NAME}.bz2"
  DUMP_URL="https://dumps.wikimedia.org/${LANGUAGE_CODE}wiki/latest/${ARCHIVE_FILE_NAME}"
}

function run {
  echo "Getting WikiExtractor"
  curl $WIKI_EXTRACTOR_URL > $WIKI_EXTRACTOR_PATH

  echo "Starting extraction for $ARCHIVE_FILE_NAME"
  extract
  cleanup
}

function extract {
  pushd $WORKSPACE

  echo "Downloading dump for $LANGUAGE_CODE at $DUMP_URL"
  curl $DUMP_URL > $WORKSPACE/$ARCHIVE_FILE_NAME
  echo "Extracting dump"
  bzip2 -d -k $WORKSPACE/$ARCHIVE_FILE_NAME
  DUMP_FILE="$WORKSPACE/$FILENAME"

  echo "Extracting with WikiExtractor"
  if [ $TYPE == "sample" ]; then
    timeout 30 python $WIKI_EXTRACTOR_PATH --processes 4 --json $DUMP_FILE || true
  elif [ $TYPE == "extract" ] || [ $TYPE == "blocklist" ]; then
    python $WIKI_EXTRACTOR_PATH --processes 4 --json $DUMP_FILE || true
  fi
  popd

  echo "Running extraction"
  pushd $PROJECT_ROOT

  if [ $TYPE == "blocklist" ]; then
    cargo run -- extract -l $LANGUAGE_CODE -d $EXTRACTED_TEXT_PATH --no_check >> $EXTRACTED_SENTENCES_PATH
  else
    cargo run -- extract -l $LANGUAGE_CODE -d $EXTRACTED_TEXT_PATH >> $EXTRACTED_SENTENCES_PATH
  fi

  popd
}

function cleanup {
  rm -rf $DUMP_PATH
  rm -rf $EXTRACTED_DUMP_PATH
}

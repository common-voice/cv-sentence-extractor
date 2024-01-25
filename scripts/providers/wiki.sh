#!/bin/bash

WIKI_EXTRACTOR_URL="https://raw.githubusercontent.com/attardi/wikiextractor/e4abb4cbd019b0257824ee47c23dd163919b731b/WikiExtractor.py"
WIKI_EXTRACTOR_PATH="$WORKSPACE/WikiExtractor.py"

function run {
  echo "Getting WikiExtractor"
  curl $WIKI_EXTRACTOR_URL > $WIKI_EXTRACTOR_PATH

  echo "Downloading Dump Listing"
  DUMP_BASE_PATH="https://dumps.wikimedia.org/${LANGUAGE_CODE}wiki/latest/"
  curl $DUMP_BASE_PATH > listing.html

  echo "Searching for correct files"
  ARCHIVE_FILE_NAME_MATCHES=($(grep -o -P -e 'wiki-latest-pages-articles-multistream\d*.xml-.*bz2"' < listing.html || [[ $? == 1 ]]))
  if [ ${#ARCHIVE_FILE_NAME_MATCHES[@]} == 0 ]; then
    ARCHIVE_FILE_NAME_MATCHES=($(grep -o -P -e 'wiki-latest-pages-articles-multistream.xml.bz2"' < listing.html))
  fi
  rm listing.html

  if [ $TYPE == "sample" ]; then
    # For a sample extract we only want to run it for the first file
    ARCHIVE_FILE_NAME=${LANGUAGE_CODE}${ARCHIVE_FILE_NAME_MATCHES/%?/}
    echo "Starting sample extraction for $ARCHIVE_FILE_NAME"
    _downloadAndDecompressDump
    extract
    cleanup
    exit $?
  fi

  for archive in "${ARCHIVE_FILE_NAME_MATCHES[@]}"
  do
    ARCHIVE_FILE_NAME=${LANGUAGE_CODE}${archive/%?/}
    echo "Starting extraction for $ARCHIVE_FILE_NAME"
    _downloadAndDecompressDump
    extract
    cleanup
  done
}

function _downloadAndDecompressDump {
  FILE_NAME=${ARCHIVE_FILE_NAME/.bz2/""}
  DUMP_URL="https://dumps.wikimedia.org/${LANGUAGE_CODE}wiki/latest/${ARCHIVE_FILE_NAME}"
  echo "Downloading dump for $LANGUAGE_CODE at $DUMP_URL"
  curl $DUMP_URL > $WORKSPACE/$ARCHIVE_FILE_NAME
  echo "Extracting dump - $ARCHIVE_FILE_NAME"
  bzip2 -d -k $WORKSPACE/$ARCHIVE_FILE_NAME
  DUMP_FILE="$WORKSPACE/$FILE_NAME"
}

function extract {
  echo "Extracting with WikiExtractor"
  if [ $TYPE == "sample" ]; then
    timeout 30 python $WIKI_EXTRACTOR_PATH --processes 4 --json -o $EXTRACTED_TEXT_PATH $DUMP_FILE || true
  elif [ $TYPE == "extract" ] || [ $TYPE == "blocklist" ]; then
    python $WIKI_EXTRACTOR_PATH --processes 4 --json -o $EXTRACTED_TEXT_PATH $DUMP_FILE
  fi

  echo "Running extraction"
  if [ $TYPE == "blocklist" ]; then
    cargo run --release -- -l $LANGUAGE_CODE -d $EXTRACTED_TEXT_PATH --no-check extract >> $EXTRACTED_SENTENCES_PATH
  elif [ -f "$TITLE_FILTER_PATH" ]; then
    cargo run --release -- -l $LANGUAGE_CODE -d $EXTRACTED_TEXT_PATH extract --title-filter-list $TITLE_FILTER_PATH >> $EXTRACTED_SENTENCES_PATH
  else
    cargo run --release -- -l $LANGUAGE_CODE -d $EXTRACTED_TEXT_PATH extract >> $EXTRACTED_SENTENCES_PATH
  fi
}

function cleanup {
  rm -rf $WORKSPACE/$ARCHIVE_FILE_NAME
  rm -rf $DUMP_FILE
  rm -rf $EXTRACTED_TEXT_PATH
}

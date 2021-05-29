#!/bin/bash

WIKI_EXTRACTOR_URL="https://raw.githubusercontent.com/attardi/wikiextractor/e4abb4cbd019b0257824ee47c23dd163919b731b/WikiExtractor.py"
WIKI_EXTRACTOR_PATH="$WORKSPACE/WikiExtractor.py"

function setup {
  DUMP_BASE_PATH="https://dumps.wikimedia.org/${LANGUAGE_CODE}wiki/latest/"
}

function _getMatchesFromListing {
  echo "Downloading Dump Listing..."
  curl $DUMP_BASE_PATH > $HERE/listing.html
  
  echo "Searching for correct files..."
  ARCHIVE_FILE_NAME_MATCHES=($(grep -o -P -e 'wiki-latest-pages-articles-multistream\d*.xml-.*bz2"' < $HERE/listing.html || [[ $? == 1 ]]))
  if [ ${#ARCHIVE_FILE_NAME_MATCHES[@]} == 0 ]; then
    ARCHIVE_FILE_NAME_MATCHES=($(grep -o -P -e 'wiki-latest-pages-articles-multistream.xml.bz2"' < $HERE/listing.html))
  fi
  
  rm $HERE/listing.html
}

function run {
  echo "Getting WikiExtractor"
  curl $WIKI_EXTRACTOR_URL > $WIKI_EXTRACTOR_PATH

  if [ -z "$ARCHIVE_FILE_NAME_MATCHES" ]; then
    _getMatchesFromListing
  fi
  
  for archive in "${ARCHIVE_FILE_NAME_MATCHES[@]}"
  do
      ARCHIVE_FILE_NAME=${archive/%?/}
      echo "Starting extraction for ... $ARCHIVE_FILE_NAME"
      dump $ARCHIVE_FILE_NAME
      extract
      cleanup
  done
}

function dump {
  DUMP_URL="${DUMP_BASE_PATH}${LANGUAGE_CODE}$1"
  FILENAME=${1/.bz2/""}
  DUMP_PATH="$WORKSPACE/$1"
  EXTRACTED_DUMP_PATH="$WORKSPACE/$FILENAME"

  echo "Downloading dump for $LANGUAGE_CODE at $DUMP_URL"
  curl $DUMP_URL > $DUMP_PATH
}

function extract {
  pushd $WORKSPACE
  echo "Extracting dump"
  bzip2 -d -k $DUMP_PATH

  echo "Extracting with WikiExtractor"
  if [ $TYPE == "sample" ]; then
    timeout 30 python $WIKI_EXTRACTOR_PATH --processes 4 --json $EXTRACTED_DUMP_PATH || true
  elif [ $TYPE == "extract" ] || [ $TYPE == "blocklist" ]; then
    python $WIKI_EXTRACTOR_PATH --processes 4 --json $EXTRACTED_DUMP_PATH || true
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

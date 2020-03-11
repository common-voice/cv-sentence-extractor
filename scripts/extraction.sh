#!/bin/bash
set -e
set -o pipefail

TYPE=${1:-sample}
HERE=$(dirname $0)
PROJECT_ROOT=$HERE/..
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
WIKI_EXTRACTOR_URL="https://raw.githubusercontent.com/attardi/wikiextractor/master/WikiExtractor.py"
WIKI_EXTRACTOR_PATH="$WORKSPACE/WikiExtractor.py"
EXTRACTED_TEXT_PATH="$WORKSPACE/text"
OUTPUT_PATH="$WORKSPACE/output"
EXTRACTED_SENTENCES_PATH="$OUTPUT_PATH/extraction-sample.txt"

mkdir -p $OUTPUT_PATH

if [ $TYPE == "sample" ]; then
  echo "Files created: $FILES_CREATED"
  echo "Files updated: $FILES_UPDATED"
  echo "Analyzing first rule file changed.."
  ALL_FILES="$FILES_CREATED $FILES_UPDATED"
  FIRST_CHANGED_RULES_FILE=( $(echo $ALL_FILES | grep -o 'src/rules/.*' || [[ $? == 1 ]]) )

  if [ ${#FIRST_CHANGED_RULES_FILE[@]} == 0 ]; then
    echo "Nothing to be done here.."
    echo "" > $EXTRACTED_SENTENCES_PATH
    exit 0
  fi

  LANGUAGE_FILE_NAME=${FIRST_CHANGED_RULES_FILE/src\/rules\//""}
  LANGUAGE_FILE_NAME=${LANGUAGE_FILE_NAME/disallowed_words\//""}
  LANGUAGE=${LANGUAGE_FILE_NAME/.toml/""}
  LANGUAGE_CODE=${LANGUAGE/.txt/""}
elif [ $TYPE == "full" ]; then
  echo "Commit: $COMMIT_MESSAGE"
  EXTRACTION_OPTION=$(echo $COMMIT_MESSAGE | grep -o -e '--full-extraction=.*$' || [[ $? == 1 ]])
  LANGUAGE_CODE=${EXTRACTION_OPTION/"--full-extraction="/""}
fi

echo "Determined that we should run an export for $LANGUAGE_CODE"

echo "Getting WikiExtractor"
curl $WIKI_EXTRACTOR_URL > $WIKI_EXTRACTOR_PATH

echo "Downloading Dump Listing..."
DUMP_BASE_PATH="https://dumps.wikimedia.org/${LANGUAGE_CODE}wiki/latest/"
curl $DUMP_BASE_PATH > listing.html

echo "Searching for correct file..."
ARCHIVE_FILE_NAME_MATCHES=$(grep -o -e 'wiki-latest-pages-articles-multistream1.xml.*bz2' < listing.html || [[ $? == 1 ]])
if [ -z $ARCHIVE_FILE_NAME_MATCHES ]; then
  ARCHIVE_FILE_NAME_MATCHES=$(grep -o -e 'wiki-latest-pages-articles-multistream.xml.bz2' < listing.html)
fi
rm listing.html

ARCHIVE_FILE_NAME=$(echo $ARCHIVE_FILE_NAME_MATCHES | head -n1 | awk '{print $1;}')


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
  elif [ $TYPE == "full" ]; then
    python $WIKI_EXTRACTOR_PATH --processes 4 --json $EXTRACTED_DUMP_PATH || true
  fi
  popd

  echo "Running extraction"
  pushd $PROJECT_ROOT
  cargo run -- extract -l $LANGUAGE_CODE -d $EXTRACTED_TEXT_PATH >> $EXTRACTED_SENTENCES_PATH
  popd
}

function cleanup {
  rm -rf $DUMP_PATH
  rm -rf $EXTRACTED_DUMP_PATH
  rm -rf $EXTRACTED_TEXT_PATH
}

function main {
  dump $ARCHIVE_FILE_NAME
  extract
  cleanup
}

main
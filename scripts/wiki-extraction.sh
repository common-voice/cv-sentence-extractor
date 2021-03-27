#!/bin/bash
set -e
set -o pipefail

# Example calls:
#   ./wiki-extraction.sh                - defaults to "sample" - used on PR - based changed files
#   ./wiki-extraction.sh extract        - full extraction - based on commit message info
#   ./wiki-extraction.sh extract en     - extracting full English

TYPE=${1:-sample}
HERE=$(dirname $0)
PROJECT_ROOT=$HERE/..
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
WIKI_EXTRACTOR_URL="https://raw.githubusercontent.com/attardi/wikiextractor/e4abb4cbd019b0257824ee47c23dd163919b731b/WikiExtractor.py"
WIKI_EXTRACTOR_PATH="$WORKSPACE/WikiExtractor.py"
EXTRACTED_TEXT_PATH="$WORKSPACE/text"
OUTPUT_PATH="$WORKSPACE/output"
mkdir -p $OUTPUT_PATH

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


if [ $TYPE == "sample" ]; then
  EXTRACTED_SENTENCES_PATH="$OUTPUT_PATH/extraction-sample.txt"

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
elif [ $TYPE == "extract" ] && [ -n "$2" ]; then
  LANGUAGE_CODE=$2
  EXTRACTED_SENTENCES_PATH="$OUTPUT_PATH/wiki.txt"
elif [ $TYPE == "extract" ]; then
  # Fallback if no language passed
  echo "Commit: $COMMIT_MESSAGE"
  LANGUAGE_CODE=$(echo $COMMIT_MESSAGE | sed -n 's/".*--full-wiki-extraction=\(\S*\)"$/\1/p' || [[ $? == 1 ]])
  EXTRACTED_SENTENCES_PATH="$OUTPUT_PATH/wiki.txt"
elif [ $TYPE == "blocklist" ] && [ -n "$2" ]; then
  LANGUAGE_CODE=$2
  EXTRACTED_SENTENCES_PATH="$WORKSPACE/full-extract-do-not-use.txt"
  touch $WORKSPACE/src/rules/$LANGUAGE_CODE.toml
fi

echo "Determined that we should run an export for $LANGUAGE_CODE"

echo "Getting WikiExtractor"
curl $WIKI_EXTRACTOR_URL > $WIKI_EXTRACTOR_PATH

echo "Downloading Dump Listing..."
DUMP_BASE_PATH="https://dumps.wikimedia.org/${LANGUAGE_CODE}wiki/latest/"
curl $DUMP_BASE_PATH > listing.html

echo "Searching for correct files..."
ARCHIVE_FILE_NAME_MATCHES=($(grep -o -P -e 'wiki-latest-pages-articles-multistream\d*.xml-.*bz2"' < listing.html || [[ $? == 1 ]]))
if [ ${#ARCHIVE_FILE_NAME_MATCHES[@]} == 0 ]; then
  ARCHIVE_FILE_NAME_MATCHES=($(grep -o -P -e 'wiki-latest-pages-articles-multistream.xml.bz2"' < listing.html))
fi
rm listing.html

for archive in "${ARCHIVE_FILE_NAME_MATCHES[@]}"
do
    ARCHIVE_FILE_NAME=${archive/%?/}
    echo "Starting extraction for ... $ARCHIVE_FILE_NAME"
    dump $ARCHIVE_FILE_NAME
    extract
    cleanup
done

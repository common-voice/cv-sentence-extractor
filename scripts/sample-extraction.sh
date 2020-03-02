#!/bin/bash
set -e
set -o pipefail

HERE=$(dirname $0)
PROJECT_ROOT=$HERE/..
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
WIKI_EXTRACTOR_URL="https://raw.githubusercontent.com/attardi/wikiextractor/master/WikiExtractor.py"
WIKI_EXTRACTOR_PATH="$WORKSPACE/WikiExtractor.py"
EXTRACTED_TEXT_PATH="$WORKSPACE/text"
OUTPUT_PATH="$WORKSPACE/output"
EXTRACTED_SENTENCES_PATH="$OUTPUT_PATH/extraction-sample.txt"

mkdir -p $OUTPUT_PATH

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
echo "Determined that we should run an export for $LANGUAGE_CODE"

DUMP_BASE_PATH="https://dumps.wikimedia.org/${LANGUAGE_CODE}wiki/latest/"

echo "Downloading Dump Listing..."
curl $DUMP_BASE_PATH > listing.html

echo "Searching for correct file..."
ARCHIVE_FILE_NAME_MATCHES=$(grep -o -e 'wiki-latest-pages-articles-multistream1.xml.*bz2' < listing.html || [[ $? == 1 ]])
if [ -z $ARCHIVE_FILE_NAME_MATCHES ]; then
  ARCHIVE_FILE_NAME_MATCHES=$(grep -o -e 'wiki-latest-pages-articles-multistream.xml.bz2' < listing.html)
fi

ARCHIVE_FILE_NAME=$(echo $ARCHIVE_FILE_NAME_MATCHES | head -n1 | awk '{print $1;}')

DUMP_URL="${DUMP_BASE_PATH}${LANGUAGE_CODE}${ARCHIVE_FILE_NAME}"
rm listing.html

FILENAME=${ARCHIVE_FILE_NAME/.bz2/""}
DUMP_PATH="$WORKSPACE/$ARCHIVE_FILE_NAME"
EXTRACTED_DUMP_PATH="$WORKSPACE/$FILENAME"

echo "Downloading dump for $LANGUAGE_CODE at $DUMP_URL"
curl $DUMP_URL > $DUMP_PATH

echo "Getting WikiExtractor"
curl $WIKI_EXTRACTOR_URL > $WIKI_EXTRACTOR_PATH

pushd $WORKSPACE
echo "Extracting dump"
bzip2 -d -k $DUMP_PATH

echo "Extracting with WikiExtractor"
timeout 30 python $WIKI_EXTRACTOR_PATH --processes 4 --json $EXTRACTED_DUMP_PATH || true
popd

echo "Running extraction"
pushd $PROJECT_ROOT
cargo run -- extract -l $LANGUAGE_CODE -d $EXTRACTED_TEXT_PATH > $EXTRACTED_SENTENCES_PATH
popd

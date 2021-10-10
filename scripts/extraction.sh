#!/bin/bash
set -e
set -o pipefail

# Example calls:
#   ./extraction.sh                - defaults to "sample" - used on PR - based changed files
#   ./extraction.sh extract        - full extraction - based on commit message info
#   ./extraction.sh extract en     - extracting full English

TYPE=${1:-sample}
HERE=$(dirname $0)
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
EXTRACTED_TEXT_PATH="$WORKSPACE/text"
OUTPUT_PATH="$WORKSPACE/output"
EXTRACTED_SENTENCES_PATH="$OUTPUT_PATH/$TYPE.txt"

mkdir -p $OUTPUT_PATH

source $HERE/providers/common.sh

echo "Installing Python dependencies"
pip3 install -r requirements.txt

if [ $TYPE == "sample" ]; then
  source $HERE/providers/wiki.sh

  setLanguageCodeFromChangedFiles

  echo "Running Wikipedia sample extraction for $LANGUAGE_CODE"
elif [ $TYPE == "blocklist" ] && [ -n "$2" ]; then
  source $HERE/providers/wiki.sh

  LANGUAGE_CODE=$2
  EXTRACTED_SENTENCES_PATH="$WORKSPACE/full-extract-do-not-use.txt"
  touch $WORKSPACE/src/rules/$LANGUAGE_CODE.toml

  echo "Running Wiki extraction to create blacklist for $LANGUAGE_CODE"
elif [ $TYPE == "extract" ] && [ -n "$2" ]; then
  source $HERE/providers/wiki.sh

  LANGUAGE_CODE=$2
  TITLE_FILTER_PATH=$3

  echo "Running Wiki extraction for $LANGUAGE_CODE"
elif [ $TYPE == "extract-wikisource" ] && [ -n "$2" ]; then
  source $HERE/providers/wiki-source.sh

  LANGUAGE_CODE=$2

  echo "Running WikiSource extraction for $LANGUAGE_CODE"
fi

run

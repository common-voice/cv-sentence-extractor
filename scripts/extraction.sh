#!/bin/bash
set -e
set -o pipefail

# Example calls:
#   ./extraction.sh                - defaults to "sample" - used on PR - based changed files
#   ./extraction.sh extract        - full extraction - based on commit message info
#   ./extraction.sh extract en     - extracting full English

TYPE=${1:-sample}
HERE=$(dirname $0)
PROJECT_ROOT=$HERE/..
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
EXTRACTED_TEXT_PATH="$WORKSPACE/text"
EXTRACTED_SENTENCES_PATH="$OUTPUT_PATH/extract.txt"
OUTPUT_PATH="$WORKSPACE/output"

mkdir -p $OUTPUT_PATH

if [ $TYPE == "sample" ]; then
  source providers/wiki.sh
  setup
  
  setLanguageCodeFromChangedFiles
  
  echo "Running Wikipedia sample extraction for $LANGUAGE_CODE"
elif [ $TYPE == "blocklist" ] && [ -n "$2" ]; then
  source providers/wiki.sh
  setup

  LANGUAGE_CODE=$2
  EXTRACTED_SENTENCES_PATH="$WORKSPACE/full-extract-do-not-use.txt"
  touch $WORKSPACE/src/rules/$LANGUAGE_CODE.toml
  
  echo "Running Wiki extraction to create blacklist for $LANGUAGE_CODE"
fi
elif [ $TYPE == "extract" ] && [ -n "$2" ]; then
  source providers/wiki.sh
  setup
  
  LANGUAGE_CODE=$2
  
  echo "Running Wiki extraction for $LANGUAGE_CODE"
fi

run

#!/bin/bash
set -e
set -o pipefail

FILE_NAME=$1
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
OUTPUT_PATH="$WORKSPACE/output"
FILE_PATH="$OUTPUT_PATH/$FILE_NAME"
OUTPUT_FILE="$OUTPUT_PATH/wiki-sorted-unique.txt"

if [[ -f "$FILE_PATH" ]]; then
  echo "Sorting and deduplicating $FILE_PATH"
  sort -u $FILE_PATH > $OUTPUT_FILE
fi

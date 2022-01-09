#!/bin/bash
set -e
set -o pipefail

FILE_NAME=$1
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
OUTPUT_PATH="$WORKSPACE/output"
FILE_PATH="$OUTPUT_PATH/$FILE_NAME"
OUTPUT_FILE="$OUTPUT_PATH/sample.txt"

if [[ -f "$FILE_PATH" ]]; then
  echo "Deduplicating $FILE_PATH"
  sort -u $FILE_PATH > $OUTPUT_FILE
  echo "Re-shuffling $OUTPUT_FILE"
  shuf $OUTPUT_FILE > $OUTPUT_FILE
fi

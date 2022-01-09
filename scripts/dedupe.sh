#!/bin/bash
set -e
set -o pipefail

FILE_NAME=$1
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
OUTPUT_PATH="$WORKSPACE/output"
FILE_PATH="$OUTPUT_PATH/$FILE_NAME"
OUTPUT_FILE="$OUTPUT_PATH/$FILE_NAME"
TEMP_FILE="$OUTPUT_PATH/temp.txt"

if [[ -f "$FILE_PATH" ]]; then
  echo "Deduplicating $FILE_PATH"
  sort -u $FILE_PATH > $TEMP_FILE
  echo "Re-shuffling $TEMP_FILE to $OUTPUT_FILE"
  shuf $TEMP_FILE > $OUTPUT_FILE
  rm -f $TEMP_FILE
fi

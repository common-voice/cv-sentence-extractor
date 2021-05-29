#!/bin/bash
set -e
set -o pipefail
set +x

# The first argument is ignored as it's the "blocklist" command
LANGUAGE_CODE=$2
FREQUENCY=$3
WORKSPACE=${GITHUB_WORKSPACE:-/tmp}
OUTPUT_PATH="$WORKSPACE/output"
mkdir -p $OUTPUT_PATH

python3 $WORKSPACE/cvtools/word_usage.py -i $WORKSPACE/full-extract-do-not-use.txt >> $OUTPUT_PATH/word_usage.$LANGUAGE_CODE.txt
python3 $WORKSPACE/cvtools/word_usage.py -i $WORKSPACE/full-extract-do-not-use.txt --max-frequency $FREQUENCY --show-words-only >> $OUTPUT_PATH/$LANGUAGE_CODE.txt

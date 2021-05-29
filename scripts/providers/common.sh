#!/bin/bash

function setLanguageCodeFromChangedFiles {
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
}

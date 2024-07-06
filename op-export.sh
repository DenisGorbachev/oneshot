#!/usr/bin/env bash

# This script takes 1 argument: ENV_FILE
# Read the $ENV_FILE

if [ $# -ne 1 ]; then
    echo "Usage: $0 <ENV_FILE>" >&2
    exit 1
fi

ENV_FILE="$1"

if [ ! -f "$ENV_FILE" ]; then
    echo "Error: $ENV_FILE does not exist or is not a file." >&2
    exit 1
fi

# Extract a list of environment variable names
ENV_VAR_NAMES=$(grep -v '^#' "$ENV_FILE" | cut -d '=' -f1)

# Read all environment variables
ENV_VARS=$(op run --env-file "$ENV_FILE" --no-masking -- printenv)

# Export all environment variables from $ENV_VARS that are in $ENV_VAR_NAMES
while IFS= read -r var_name; do
    var_value=$(echo "$ENV_VARS" | grep "^$var_name=" | cut -d '=' -f2-)
    if [ -n "$var_value" ]; then
        printf 'export %s=%q\n' "$var_name" "$var_value"
    fi
done <<< "$ENV_VAR_NAMES"

#!/bin/bash

function extract_bytes() {
    local start=$1
    local end=$2
    local file=$3

    # Calculate the byte count to extract
    local byte_count=$((end - start + 1))

    # Use dd to extract the specified byte range
    dd if="$file" of=/dev/stdout bs=1 skip="$start" count="$byte_count"
}

# Check for proper number of arguments
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 start_bytes end_bytes filename"
    exit 1
fi

# Call the function with command line arguments
extract_bytes $1 $2 $3


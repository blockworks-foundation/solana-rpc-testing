#!/bin/bash

# exit on error
set -e

# directory and url
directory="metrics/$1"
url="$2"

# write a function to print error message in red color and exit
function error_exit() {
    echo -e "\e[31m$1\e[0m"
    exit 1
}

# write a function to print message in green color
function info() {
    echo -e "\e[32m$1\e[0m"
}

# exit error if $1 is not provided
if [ -z "$1" ]; then
    echo "Error: Please provide a directory name"
    exit 1
fi

# set url to localhost if not provided
if [ -z "$url" ]; then
    url="http://localhost:8899"
fi

# Fetch hard limit
hard_limit=$(ulimit -Hn)

# Soft limit 9/10th of hard limit
soft_limit=$((hard_limit * 9 / 10))
# set
ulimit -Sn "$soft_limit"

info "Hard limit: $hard_limit"
info "Soft limit: $(ulimit -Sn)"

# check for directory
if [ ! -d "$directoy" ]; then
    info "Creating directory: $directory"
    mkdir -p "$directory"
else
    error_exit "Directory already exists: $directory"
fi

# airdrop
info "Airdropping..."
solana airdrop 100000 -u "$url"
# configure
info "Configuring..."
yarn configure -u "$url"
# run bench
info "Running bench..."
./bench-rpc-by-threads.sh "$directory" -r "$url"

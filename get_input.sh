#!/usr/bin/bash

AOC_SESSION='53616c7465645f5fb98358f4b739bef70766684ded9896d643bded915423c8768ad0295c7c9cc7a71bc872e83f8abdf8105f1d9b666672f5fb9a05660c650982'
main_dir="./"

today="${1:-$(date +"%-e")}"
dist="$main_dir/data/$today"
file="$dist/input"

if [[ ! -d "$main_dir/$today/data" ]]; then
	mkdir -p "$dist"
fi

curl --cookie "session=$AOC_SESSION" "https://adventofcode.com/2024/day/$today/input" > "$file"

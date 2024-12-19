#!/usr/bin/bash

AOC_SESSION="$(pass aoc_session)"

today="${1:-$(date +"%-e")}"
dir="data/$today"

if [[ ! -d "$dir" ]]; then
	mkdir -p "$dir"
elif [[  -f "$dir/input" ]]; then
	rm "$dir/input" && echo "old file removed"
fi

curl --cookie "session=$AOC_SESSION" "https://adventofcode.com/2024/day/$today/input" > "$dir/input"

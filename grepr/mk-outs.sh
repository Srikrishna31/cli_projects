#!/usr/bin/env bash

set -u

DIR="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"
rm -f "$OUT_DIR/*"

# Empty file
grep foo $DIR/empty.txt > "$OUT_DIR/foo.empty.txt"

# Empty regex
grep "" $DIR/fox.txt > "$OUT_DIR/empty_regex.fox.txt"

# Case-sensitive
grep The $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.capitalized"
grep the $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase"
grep -i the $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase.insensitive"
grep nobody $DIR/nobody.txt > "$OUT_DIR/nobody.txt"
grep -i nobody $DIR/nobody.txt > "$OUT_DIR/nobody.txt.insensitive"

# Case-sensitive multiple files
grep The $DIR/*.txt > "$OUT_DIR/all.the.capitalized"
grep -i the $DIR/*.txt > "$OUT_DIR/all.the.lowercase.insensitive"

# Recursive, handle directory
grep -r dog tests/inputs > "$OUT_DIR/dog.recursive"

# Recursive, insensitive
grep -ri then tests/inputs > "$OUT_DIR/the.recursive.insensitive"

# Case-sensitive count
grep -c The $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.capitalized.count"
grep -c the $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase.count"
grep -ci the $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase.insensitive.count"
grep -c nobody $DIR/nobody.txt > "$OUT_DIR/nobody.txt.count"
grep -ci nobody $DIR/nobody.txt > "$OUT_DIR/nobody.txt.insensitive.count"

# Case-sensitive, count, multiple files
grep -c The $DIR/*.txt > "$OUT_DIR/all.the.capitalized.count"
grep -ci the $DIR/*.txt > "$OUT_DIR/all.the.lowercase.insensitive.count"

# Recursive, insensitive count
grep -cri the "$DIR/" > "$OUT_DIR/the.recursive.insensitive.count"

# STDIN, insensitive, count
cat $DIR/*.txt | grep -ci the - > "$OUT_DIR/the.recursive.insensitive.count.stdin"

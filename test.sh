#!/usr/bin/env bash

for DIR in *; do
  # Only run tests on directories that are not "big" and do not contain "utils" and also are directories
  if [ -d "$DIR" ] && [[ "$DIR" != "big"* ]] && [[ "$DIR" != *"utils" ]];

  then
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd "$DIR" && cargo test -q > /dev/null && cargo clippy)
  fi
done

echo "Done."
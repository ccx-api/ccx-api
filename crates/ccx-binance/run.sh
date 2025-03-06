#!/usr/bin/env bash

EXAMPLE_BIN="${1}"

if [ -z "${EXAMPLE_BIN}" ]; then
    echo "Usage: $0 <example-bin>"
    # list available examples
    echo "Available examples:"
    find examples -type f -name '*.rs' -print0 | xargs -0 -n1 basename | sed 's/\.rs//'
    exit 1
fi

cargo build \
  --example "${EXAMPLE_BIN}"

"../../target/debug/examples/${EXAMPLE_BIN}"

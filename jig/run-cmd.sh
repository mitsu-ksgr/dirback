#!/usr/bin/env bash
#
# Run cmd crate with DIRBACK_STORE_DIR env var.
#
readonly APP_DATA_PATH="$(dirname $0)/../tmp/cmd-test"

if [ ! -d "${APP_DATA_PATH}" ]; then
    mkdir -p "${APP_DATA_PATH}"
fi

DIRBACK_STORE_DIR="${APP_DATA_PATH}" \
    cargo run --bin cmd -- $@


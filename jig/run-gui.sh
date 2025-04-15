#!/usr/bin/env bash
#
# Run gui crate with datadir option.
#

# move to the project root.
cd $(dirname $0)/..

readonly APP_DATA_PATH="./tmp/cmd-test"

if [ ! -d "${APP_DATA_PATH}" ]; then
    mkdir -p "${APP_DATA_PATH}"
fi

DIRBACK_STORE_DIR="${APP_DATA_PATH}" \
    cargo run --bin dirback_gui -- -d ${APP_DATA_PATH}


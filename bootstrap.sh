#! /usr/bin/env bash

SCRIPT_PATH=$(cd "$(dirname "$0")" ; pwd -P)

pushd "${SCRIPT_PATH}" > /dev/null
  # clear an existing database
  rm -f "${DATABASE_URL}"

  # install the diesel CLI tools if they're not installed
  if ! command -v diesel >/dev/null 2>&1; then
    cargo install diesel_cli --version '<= 1.2' --no-default-features --features=postgre > /dev/null
  fi

  # create db/db.postgre
  diesel migration --database-url="${DATABASE_URL}" run > /dev/null
popd > /dev/null

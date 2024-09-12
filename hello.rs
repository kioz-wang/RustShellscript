#!/bin/env bash
#![allow(unused_attributes)] /*
declare -r SOURCE_PATH="$0"
# shellcheck disable=SC2155
declare -r SOURCE_NAME=$(basename "${SOURCE_PATH}")
# shellcheck disable=SC2155
declare -ra TARGETS_PATH=$(ls -t /tmp/"${SOURCE_NAME}".* 2>/dev/null)
declare TARGET_PATH="${TARGETS_PATH[0]}"
if [ "${SOURCE_PATH}" -nt "${TARGET_PATH}" ]; then
    TARGET_PATH=$(mktemp --dry-run /tmp/"${SOURCE_NAME}".XXXXX)
    rustc "${SOURCE_PATH}" -o "${TARGET_PATH}" || exit $?
fi
exec "${TARGET_PATH}" "$@" || exit $? # */

use std::env;

fn main() {
    println!("hello {:?}", env::args().collect::<Vec<_>>());
}

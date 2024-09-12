# Rust Shellscript Lang

Run single `Rust` source like a `Shellscript`.

For `Rust` source, suffix is `.rs`, and for `Rustshellscript` source, suffix is same as `Rust`! But if you like, use `.rss`.

When run `.rss`, it would be compiled first, then executed.

- all executables would be not removed
- run again without modification, just execute
- run again with some modifications, recompile, then execute

## Dependencies

- rustc
- basename
- mktemp

## Howto Use

1. Insert `Rustshellscript` magic codes in header of single `Rust` source
2. Now you get a single `Rustshellscript` source
3. Give executable permission
4. Run it!

## Magic Codes

### for Bash

```bash
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
```

### for Ksh

```ksh
# should be same as `for Bash`
```

### for more ...

```shell
# TBD
```

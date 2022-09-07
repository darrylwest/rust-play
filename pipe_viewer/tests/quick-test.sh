#!/usr/bin/env bash
# dpw@LanisAppleWatch.localdomain
# 2022-09-07 14:29:53
#

set -eu

clear && cargo build && printf "\n\n\n" && RUST_LOG=info ./target/debug/pipe_viewer --input-file tests/test-data.dat --output-file tests/test-data.out

exit $?


#!/usr/bin/env bash
# dpw@tiburon.local
# 2023-12-08 18:00:00
#

set -eu

echo "/now" | nc -w 1 -u 127.0.0.1 22200

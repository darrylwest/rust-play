#!/usr/bin/env bash
# dpw@tiburon.local
# 2023-12-08 17:59:38
#

set -eu

echo "/ping" | nc -w 1 -u 127.0.0.1 22200

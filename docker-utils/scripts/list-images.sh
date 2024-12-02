#!/usr/bin/env bash
# dpw@alamo
# 2024-12-02 15:18:56
#

set -eu

curl --unix-socket /var/run/docker.sock http://localhost/v1.47/images/json | jq

exit $?


#!/usr/bin/env bash
# dpw@alamo
# 2024-12-02 15:16:13
#

set -eu

# curl --unix-socket /var/run/docker.sock http://localhost/v1.47/images/json | jq
curl --unix-socket /var/run/docker.sock http://localhost/v1.47/containers/json | jq

exit $?


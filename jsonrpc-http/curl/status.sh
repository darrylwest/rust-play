#!/usr/bin/env bash
# dpw@piedmont
# 2021.09.25
#
# Style Guide Reference: https://google.github.io/styleguide/shellguide.html
#

set -eu

. curl/config.sh

id=$(txid)
printf -v request '{"jsonrpc":"2.0", "method":"api.v1.status","id":"%s"}' $id

echo "Request:"
echo $request | jq '.'

echo "Response:"
curl -s -X POST -H "Content-Type: application/json" -d "$request" $HOST | jq '.'

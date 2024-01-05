#!/usr/bin/env bash
# dpw@plaza.local
# 2024-01-04 18:47:59
#

set -eu

file=scripts/data/table.csv
if [ $# -eq 1 ]
then
    file=$1
else
    echo ""
    echo "This is the default file; pass in an alternate csv file..."
    echo ""
fi

csview --style Markdown $file




#!/usr/bin/env bash
# dpw@alamo
# 2023-12-18 17:51:09
#

set -eu

SS=`git status -s`

if [[ -z $SS ]]
then
    echo "clean"
else
    echo "not clean..."
    exit 0
fi



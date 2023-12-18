#!/usr/bin/env bash
# dpw@alamo
# 2023-12-18 17:51:09
#

set -eu

SS=`git status -s`

if [[ -z $SS ]]
then
    echo "clean"
    git co main && git pull && git merge develop && git push && git co develop
else
    echo "Abort: local repo not clean..."
    git status
fi



#!/usr/bin/env bash
# dpw@tacoma
# 2021.08.03
#
    
set -eu

echo "consider using rustup?"

exit 0

curl https://sh.rustup.rs -sSf | sh

- or -

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y


#!/usr/bin/env sh

if [ "$1" = "functional" ]; then
    k6 run ./test/functional.js
    exit
fi

if [ "$1" = "load" ]; then
    k6 run ./test/load/constant-vus.js
    exit
fi

k6 $@

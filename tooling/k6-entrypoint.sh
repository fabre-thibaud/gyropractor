#!/usr/bin/env sh

if [ "$1" = "functional" ]; then
    k6 run ./test/functional-test.js
    exit
fi

if [ "$1" = "load" ]; then
    k6 run ./test/health-test.js
    exit
fi

k6 $@

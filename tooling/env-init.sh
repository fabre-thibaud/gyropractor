#!/bin/bash

if [ ! -f "./.env" ]; then
    echo "no .env found, creating from dist file"
    cp ./.env{.dist,}
fi

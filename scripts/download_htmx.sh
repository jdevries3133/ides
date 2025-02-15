#!/bin/sh

set -eux

HTMX_CHECKSUM="e1746d9759ec0d43c5c284452333a310bb5fd7285ebac4b2dc9bf44d72b5a887"
HTMX_VERSION="2.0.2"

HTMX_PATH=website/src/htmx-$HTMX_VERSION.vendor.js

if [ ! -f ./website/src/htmx-$HTMX_VERSION.vendor.js ]
then
    curl -L https://unpkg.com/htmx.org@$HTMX_VERSION > $HTMX_PATH
fi

checksum="$(
    openssl dgst -sha256 -hex $HTMX_PATH \
    | sed 's/.*= \(.*\)/\1/g'
)"

if [ "$checksum" != "$HTMX_CHECKSUM" ]; \
then
    rm $HTMX_PATH
    exit 1
fi

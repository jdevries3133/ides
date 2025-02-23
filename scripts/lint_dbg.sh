#!/bin/sh

if [ ! -z "$(git ls-files | grep '\.rs$' | xargs -I{} grep 'dbg!' {})" ]
then
    echo "Fatal: found lingering dbg! statements!"
    exit 1
fi

#!/usr/bin/env bash

set -ex

rm -rf target/redoxer
mkdir -p target/redoxer

redoxer install \
    --no-track \
    --path examples/editor-orbclient \
    --root "target/redoxer"

cmd="env RUST_LOG=cosmic_text=debug,editor_orbclient=debug ./bin/editor-orbclient"
if [ -f "$1" ]
then
    filename="$(basename "$1")"
    cp "$1" "target/redoxer/${filename}"
    cmd="${cmd} '${filename}'"
fi

cd target/redoxer

# TODO: remove need for linking fonts
redoxer exec \
    --gui \
    --folder . \
    /bin/sh -c \
    "${cmd}"

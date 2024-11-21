#!/usr/bin/env bash

# Script to re-vendor the WIT files.
#
# This script is also executed on CI to ensure that everything is up-to-date.
set -ex

# Space-separated list of wasi proposals that are vendored here along with the
# tag that they're all vendored at.
#
# This assumes that the repositories all have the pattern:
# https://github.com/WebAssembly/wasi-$repo
# and every repository has a tag `v$tag` here. That is currently done as part
# of the WASI release process.

tag=$(curl -sL https://api.github.com/repos/WebAssembly/Wasi/releases/latest | jq -r .tag_name | sed 's/^v//')

repos="cli clocks filesystem http io random sockets"
dst=./wit/deps

rm -rf $dst
mkdir -p $dst
echo $tag > ./wit/deps/wit-ver.txt

for repo in $repos; do
  mkdir $dst/$repo
  curl -sL https://github.com/WebAssembly/wasi-$repo/archive/refs/tags/v$tag.tar.gz | \
    tar xzf - --strip-components=2 -C $dst/$repo wasi-$repo-$tag/wit
  rm -rf $dst/$repo/deps*
done

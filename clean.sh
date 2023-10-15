#/bin/bash

#set -e 
set -x 

pushd rust
cargo clean
popd

rm -rf ./c/build

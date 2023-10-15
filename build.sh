#/bin/bash

#set -e 
set -x 

# Build rust library
pushd "./rust"
cargo build 
popd 

# Build c program
pushd "./c"
if [ -d ./build ]; then
    rm -rf "./build"
fi
mkdir "./build"
pushd "./build"

lex ../src/lex.l                           
yacc -v -d ../src/yacc.y
clang lex.yy.c y.tab.c ../src/main.c -L ../../rust/target/debug -lrust -o coop

popd
popd

mv ./c/build/coop .

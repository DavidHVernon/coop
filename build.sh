#/bin/bash

#set -e 
set -x 

rm lex.yy.c y.tab.c y.tab.h y.output

lex lex.l                           
yacc -v -d yacc.y

clang lex.yy.c y.tab.c y.tab.c -o s


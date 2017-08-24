#!/usr/bin/env bash

clang -O2 -emit-llvm -c $1 -o - | llc -march=bpf -filetype=obj -o "`basename $1 .c`.o"

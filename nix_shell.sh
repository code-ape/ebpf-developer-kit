#!/usr/bin/env bash

nix-shell \
    -p rustChannels.stable.rust \
    -p gcc \
    -p llvm_38 \
    -p clang_38

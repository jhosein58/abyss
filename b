#!/usr/bin/env bash

./abyssc
gcc -O3 -march=x86-64 -fomit-frame-pointer -funroll-loops tmp/out.c -o tmp/out
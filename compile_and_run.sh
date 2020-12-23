#!/bin/bash

time rustc -o build/main main.rs

if [ $? -eq 0 ]; then
  ./build/main
fi

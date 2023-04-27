#!/bin/bash

mkdir -p bin

case $1 in
  --help)
    echo "Try 'run.sh 1' or 'run.sh 2'"
    ;;
  1) 
    c++ dangling_reference.cpp -o bin/dangling_reference && ./bin/dangling_reference
    ;;
  2) 
    c++ use_after_move.cpp -o bin/use_after_move && ./bin/use_after_move
    ;;
esac
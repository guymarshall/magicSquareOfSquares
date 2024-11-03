#!/bin/bash

gcc -o get_most_frequent_total get_most_frequent_total.c -O3

if [ $? -eq 0 ]; then
    mkdir -p bin

    mv get_most_frequent_total bin/

    ./bin/get_most_frequent_total
else
    echo "Compilation failed."
fi
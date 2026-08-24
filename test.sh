#!/bin/bash

fn_print_usage() {
    printf "\nUSAGE: ./test.sh <bin>\n\nThis executable will use testcases provided in tests dir to test the specified rust binary"
}

if [[ -z "$1" ]]; then
    fn_print_usage
    exit 1
fi

if [[ -e "tests/$1" ]]; then
    printf "Running \e[0;34m$1\e[0m\n"
    declare -i i
    declare -i good
    i=1
    good=0
    while [[ -e "tests/$1/$i.in" ]]; do
        printf "\e[0;33mRunning test case $i: \e[0m"
        OUT=$(cargo run -q --bin "$1" < "tests/$1/$i.in")
        if [[ "$( < "tests/$1/$i.out")" == "$OUT" ]]; then
            printf "\e[1;32mSUCCESS\e[0m\n\n"
            good+=1
        else
            printf "\e[1;31mFAILED\e[0m\n\n"
        fi
        i+=1
    done

    total=$((i - 1))
    printf "RESULTS: $good/$total tests passed, $((total-good))/$total tests failed\n"
else
    printf "\nNo testcases found for specified binary\n"
    fn_print_usage
fi

#!/bin/bash
output=$(pls)
if [[ $output == cd* ]]; then
    eval "$output"
fi

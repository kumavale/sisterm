#!/bin/bash

gcc sisterm.c -o sisterm && echo "compile sccess!"; echo

if [ $# -eq 1 ]; then
  if [ $1 = "run" ]; then
    sudo ./sisterm /dev/ttyS5
  fi
fi

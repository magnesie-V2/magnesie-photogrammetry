#!/bin/bash

ID="$1"
shift

LOG_FILE="/logs/job/${ID}_perf"

# Delete 2 first lines of perf stat report, delete all after ,Joules for each line, delete all spaces
sed -e '1,2d; s/,Joules.*//; s/.* //' $LOG_FILE
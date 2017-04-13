#!/bin/sh

BASE=/media/sf_C_DRIVE/Users/foriequal0/Downloads/codejam/
PROBLEM=$1
FILENAME=$2

stack run -- -- ${PROBLEM} \
      < "${BASE}/${FILENAME}.in" \
      > "${BASE}/${FILENAME}.out"

cp src/${PROBLEM}.hs \
   ${BASE}/

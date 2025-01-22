#!/bin/bash

source env.sh

for i in {0..9}
do
    echo "Starting n$i"
    incus start n$i --project $INCUS_PROJECT_NAME
done

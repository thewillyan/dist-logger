#!/bin/bash

source env.sh

for i in {0..9}
do
    echo "Stopping n$i"
    incus stop n$i --project $INCUS_PROJECT_NAME
done

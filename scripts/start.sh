#!/bin/bash

source env.sh

for i in {0..9}
do
    echo "Starting n$i"
    lxc start n$i --project $LXC_PROJECT_NAME
done

#!/bin/bash

source env.sh

for i in {0..9}
do
    echo "Stopping n$i"
    lxc stop n$i --project $LXC_PROJECT_NAME
done

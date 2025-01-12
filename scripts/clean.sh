#!/bin/bash

source env.sh

# Delete all node containers
for i in {0..9}
do
    echo "Deleting n$i"
    lxc delete n$i -f --project $LXC_PROJECT_NAME
done

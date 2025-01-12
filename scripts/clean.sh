#!/bin/bash

LXC_PROJECT_NAME="dist-logger"

# Delete all node containers
for i in {0..9}
do
    echo "Deleting n$i"
    lxc delete n$i -f --project $LXC_PROJECT_NAME
done

#!/bin/bash

source env.sh

# Restores all node containers to the initial state
for i in {0..9}
do
    echo "Restoring n$i"
    lxc restore n$i initial-state --project $LXC_PROJECT_NAME
done

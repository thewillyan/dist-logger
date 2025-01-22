#!/bin/bash

source env.sh

# Delete all node containers
for i in {0..9}
do
    echo "Deleting n$i"
    incus delete n$i -f --project $INCUS_PROJECT_NAME
done

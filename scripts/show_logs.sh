#!/bin/bash

source env.sh

for i in {0..9}
do
    node="n$i"
    log="/root/$node.log"

    incus exec "$node" --project "$INCUS_PROJECT_NAME" -- bash -c "wc -l $log && cat $log"
done

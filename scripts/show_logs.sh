#!/bin/bash

source env.sh

for i in {0..9}
do
    node="n$i"
    log="/root/$node.log"

    lxc exec "$node" --project "$LXC_PROJECT_NAME" -- bash -c "wc -l $log && cat $log"
done

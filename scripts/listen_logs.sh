#!/bin/bash

for i in {0..9}
do
    lxc exec n$i --project dist-logger -- tail -f n$i.log &
done

wait

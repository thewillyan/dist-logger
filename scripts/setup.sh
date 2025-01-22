#!/bin/bash

source env.sh

# setup isolated project
if ! incus project list --format=csv | cut -d ',' -f 1 | grep -q "$INCUS_PROJECT_NAME"
then
    echo "Creating" $INCUS_PROJECT_NAME "project" 
    incus project create $INCUS_PROJECT_NAME
    incus profile device add default root disk path=/ pool=default --project $INCUS_PROJECT_NAME
    incus network create lxdbr-dl --project $INCUS_PROJECT_NAME
    incus profile device add default eth0 nic name=eth0 type=nic network=lxdbr-dl --project $INCUS_PROJECT_NAME
fi

# create node containers
for i in {0..9}
do
    incus launch $IMAGE n$i --project $INCUS_PROJECT_NAME --network lxdbr-dl
    incus config set n$i boot.autostart false --project $INCUS_PROJECT_NAME
    incus snapshot create n$i initial-state --project $INCUS_PROJECT_NAME
done

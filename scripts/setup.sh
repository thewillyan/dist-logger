#!/bin/bash

source env.sh

# setup isolated project
if ! lxc project list --format=csv | cut -d ',' -f 1 | grep -q "$LXC_PROJECT_NAME"
then
    echo "Creating" $LXC_PROJECT_NAME "project" 
    lxc project create $LXC_PROJECT_NAME
    lxc profile device add default root disk path=/ pool=default --project $LXC_PROJECT_NAME
    lxc profile device add default eth0 nic name=eth0 nictype=p2p --project $LXC_PROJECT_NAME
fi

# create node containers
for i in {0..9}
do
    lxc launch $IMAGE n$i --project $LXC_PROJECT_NAME
done

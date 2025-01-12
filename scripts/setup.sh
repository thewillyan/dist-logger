#!/bin/bash

source env.sh

# setup isolated project
echo "Creating" $LXC_PROJECT_NAME "project" 
lxc project create $LXC_PROJECT_NAME
lxc profile device add default root disk path=/ pool=default --project $LXC_PROJECT_NAME
lxc profile device add default eth0 nic name=eth0 nictype=p2p --project $LXC_PROJECT_NAME

# create node containers
for i in {0..9}
do
    lxc launch $IMAGE n$i --project $LXC_PROJECT_NAME
done

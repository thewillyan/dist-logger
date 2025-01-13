#!/bin/bash

source env.sh

# Delete all node containers
for i in {0..9}
do
    echo "Deleting n$i"
    lxc delete n$i -f --project $LXC_PROJECT_NAME
done

# Delete all images
fingerprints=$(lxc image list --project $LXC_PROJECT_NAME --format=csv | cut -d ',' -f 2)
for fingerprint in $fingerprints
do
    echo "Deleting image $fingerprint"
    lxc image delete $fingerprint --project $LXC_PROJECT_NAME
done

# Delete project
echo "Deleting project $LXC_PROJECT_NAME"
lxc project delete $LXC_PROJECT_NAME

# Delete network
lxc network delete lxdbr-dl

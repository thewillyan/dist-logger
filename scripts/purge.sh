#!/bin/bash

source env.sh

# Delete all node containers
for i in {0..9}
do
    echo "Deleting n$i"
    incus delete n$i -f --project $INCUS_PROJECT_NAME
done

# Delete all images
fingerprints=$(incus image list --project $INCUS_PROJECT_NAME --format=csv | cut -d ',' -f 2)
for fingerprint in $fingerprints
do
    echo "Deleting image $fingerprint"
    incus image delete $fingerprint --project $INCUS_PROJECT_NAME
done

# Delete project
echo "Deleting project $INCUS_PROJECT_NAME"
incus project delete $INCUS_PROJECT_NAME

# Delete network
incus network delete lxdbr-dl

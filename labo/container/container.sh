#!/bin/bash

ROOTFS=$(mktemp -d)
echo ROOTFS: $ROOTFS
CID=$(sudo docker container create bash)
echo CID: $CID
sudo docker container export $CID | tar -x -C $ROOTFS
ln -s /usr/local/bin/bash $ROOTFS/bin/bash
sudo docker container rm $CID

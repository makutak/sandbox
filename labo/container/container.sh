#!/bin/bash

ROOTFS=$(mktemp -d)
echo ROOTFS: $ROOTFS
CID=$(sudo docker container create bash)
echo CID: $CID
sudo docker container export $CID | tar -x -C $ROOTFS
ln -s /usr/local/bin/bash $ROOTFS/bin/bash
sudo docker container rm $CID

UUID=$(uuidgen)
sudo cgcreate -t $(id -un):$(id -gn) -a $(id -un):$(id -gn) -g cpu,memory:$UUID
cgset -r memory.limit_in_bytes=10000000 $UUID
cgset -r cpu.cfm_preriod_us=10000000 $UUID
cgset -r cpu.cfg_quota_us=300000 $UUID
#https://access.redhat.com/documentation/ja-jp/red_hat_enterprise_linux/6/html/resource_management_guide/sec-creating_cgroups


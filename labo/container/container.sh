#!/bin/bash

ROOTFS=$(mktemp -d)
echo ROOTFS: $ROOTFS
CID=$(sudo docker container create bash)
echo CID: $CID
sudo docker container export $CID | tar -x -C $ROOTFS
ln -s /usr/local/bin/bash $ROOTFS/bin/bash
sudo docker container rm $CID

UUID=$(uuidgen)
echo UUID: $UUID
sudo cgcreate -t $(id -un):$(id -gn) -a $(id -un):$(id -gn) -g cpu,memory:$UUID
cgset -r memory.limit_in_bytes=10000000 $UUID
cgset -r cpu.cfs_period_us=1000000 $UUID
cgset -r cpu.cfs_quota_us=30000 $UUID
#https://access.redhat.com/documentation/ja-jp/red_hat_enterprise_linux/6/html/resource_management_guide/sec-creating_cgroups

CMD="/bin/sh"
cgexec -g cpu,memory:$UUID \
  unshare -muinpfr /bin/sh -c "
    mount -t proc proc $ROOTFS/proc &&
    touch $ROOTFS$(tty); mount --bind $(tty) $ROOTFS$(tty) &&
    touch $ROOTFS/dev/pts/ptmx; mount --bind /dev/pts/ptmx $ROOTFS/dev/pts/ptmx &&
    ln -sf /dev/pts/ptmx $ROOTFS/dev/ptmx &&
    touch $ROOTFS/dev/null && mount --bind /dev/null $ROOTFS/dev/null &&
    /bin/hostname $UUID &&
    exec capsh --chroot=$ROOTFS --drop=cap_sys_chroot -- -c 'exec $CMD'
   "

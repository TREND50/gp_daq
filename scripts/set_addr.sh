#!/bin/sh

if [ $# -lt 2 ]
then
    echo "Usage: " $0 "<ifname> <macfile>"
    exit
fi
./gen_ip_cfg.sh $1 $2 >addr.yaml 
sudo ../target/release/set_addr $1 ./addr.yaml 1234 8888

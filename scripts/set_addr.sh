#!/bin/sh

if [ $# -lt 1 ]
then
    echo "Usage: " $0 "<ifname>"
    exit
fi

sudo ../target/release/set_addr $1 ./addr1.yaml 1234 888

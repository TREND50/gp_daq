#!/bin/sh

function get_ip {
    ip address show $1 |grep 'inet\ '  |awk '{print $2}'|awk -F / '{print $1}'
}

function get_mac {
    cat /sys/class/net/$1/address
}

if [ $# -eq 0 ]
then
    ls /sys/class/net
else
    for i in $@
    do
	if [ -d /sys/class/net/$i ]
	then
	    echo $i
	fi
    done
fi|while read i
do
    echo $i `get_mac $i` `get_ip $i`
done

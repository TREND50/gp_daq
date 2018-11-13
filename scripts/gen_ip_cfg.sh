#!/bin/sh

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

if [ $# -lt 2 ]
then
    echo "Usage: $0 <iface name> <mac ip file>"
    exit
fi

iface=$1

if [ -d /sys/class/net/${iface} ]
then
    :
else
    exit -1
fi


addr=`${SCRIPTPATH}/get_addr.sh $iface`
mac_addr=`echo $addr|awk '{print $2}'|tr -d ':'`
ip_addr=`echo $addr |awk '{print $3}'|awk -F \. '{printf("%s, %s, %s, %s", $1, $2, $3, $4)}'`
subnet_addr=`echo $addr |awk '{print $3}'|awk -F \. '{printf("%s, %s, %s", $1, $2, $3)}'`

shift

for f in $@
do
    cat $f |while read i
    do
	board_mac=`echo $i|awk '{print $1}'|tr -d ':'`
	board_id=`echo $i |awk '{print $2}'`
cat <<EOF
msg_type: INTREG
write: 1
board_mac: 0x$board_mac
board_ip: [ ${subnet_addr} ,  $board_id ]
srv_mac1:  0x${mac_addr}
srv_ip1: [ ${ip_addr} ]
srv_mac2:  0x${mac_addr}
srv_ip2: [ ${ip_addr} ]
port1: 1236
port2: 1235
EOF
echo ---
    done
done

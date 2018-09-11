#!/bin/bash

SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
CFG_DIR=$SELF_DIR/../cfgs

DAQ_DIR=$SELF_DIR/../

if [ "x$DATADIR" = x ]
then
    echo "set DATADIR first"
    exit
fi


if [ $# != 1 ]
then
    echo "Usage:" $0 "<board ID>"
    exit
fi

MON_PORT=6666
# Configuration
BOARDID=$1
if [ -f $DATADIR/last_run.txt ]
then
:
else
    echo 0 > $DATADIR/last_run.txt
fi


NRUN=$(<$DATADIR/last_run.txt)
NRUN=$(($NRUN+1))

# Clean
tmux kill-window -t "w"  
# Execute run
${SELF_DIR}/run.sh  1236 192.168.1.1${BOARDID} ${CFG_DIR}/minbias.yaml  $DATADIR/M$NRUN'_b'$BOARDID.data "w" 1
#for i in {1..1000}
#do 
#  echo 'Event' $i
#done

# Log run id
cp minbias.cfg  $DATADIR/M$NRUN'_b'$BOARDID.cfg
rm $DATADIR/last_run.txt
echo $NRUN >> $DATADIR/last_run.txt
echo "Now killing tmux window w." 
#tmux kill-window -t "w"

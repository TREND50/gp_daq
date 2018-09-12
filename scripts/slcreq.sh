#!/bin/sh
# Pattern run
# Parameters: pattern mode
SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
CFG_DIR=$SELF_DIR/../cfgs
DAQ_DIR=$SELF_DIR/../

if [ "x$DATADIR" = x ]
then
    echo "set DATADIR first"
    exit
fi



if [ $# != 2 ]
then
    echo "Usage:" $0 "<board ID> <loop>"
    exit
fi

if [ -f $DATADIR/last_run.txt ]
then
:
else
    echo 0 > $DATADIR/last_run.txt
fi


# Configuration
BOARDID=$1
NRUN=$(<$DATADIR/last_run.txt)
echo 'Present run ID' $NRUN
#NRUN=$(($NRUN+1))  No run increment for SLC request


# Execute run
$SELF_DIR/run.sh  1235 192.168.1.1$BOARDID $CFG_DIR/slcreq.yaml $DATADIR/S$NRUN'_b'$BOARDID.data "wslc" $2  # Ask for a loop
echo "Now killing server wslc."
tmux kill-window -t "wslc"

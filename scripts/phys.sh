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


if [ $# != 8 ]
then
    echo "Usage:" $0 "<board ID> <Th1-> <Th1+> <Th2-> <Th2+> <Th3-> <Th3+>  <TrigPattern>"
    exit
fi

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

python $SELF_DIR/fill_phys_yaml.py ${CFG_DIR}/physgen.yaml $2 $3 $4 $5 $6 $7 $8 $DATADIR/phys${NRUN}.yaml


# Clean
tmux kill-window -t "w"
# Execute run
$SELF_DIR/run.sh  1236 192.168.1.1${BOARDID} $DATADIR/phys${NRUN}.yaml $DATADIR/R$NRUN'_b'$BOARDID.data "w" 0

# Log run id
mv $DATADIR/phys${NRUN}.yaml  $DATADIR/R$NRUN'_b'$BOARDID.yaml
rm $DATADIR/last_run.txt
echo $NRUN >> $DATADIR/last_run.txt
#echo "Now killing tmux window w." 

#!/bin/sh
# Runs soft triggers

SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
PROG_DIR=$SELF_DIR/../target/release/
CFG_DIR=$SELF_DIR/../cfgs
#export DATADIR=/mnt/disk/

if [ "x$DATADIR" = x ]
then
    echo "set DATADIR first"
    exit
fi


# Configuration
BOARDID=$1

echo "Nb of paras =" $#
if [ $# == 1 ]
then
  if [ -f $DATADIR/last_run.txt ]
  then
  :
  else
    echo 0 > $DATADIR/last_run.txt
  fi
  NRUN=$(<$DATADIR/last_run.txt)
  NRUN=$(($NRUN+1))
elif [ $# == 9 ]
then
  echo 
  NRUN=$9
else
  echo "Usage:" $0 "<board ID>"
  exit 
fi


echo "NRUN=" $NRUN

SLC_FILE=$DATADIR/S${NRUN}.yaml
DATA_FILE=$DATADIR/M${NRUN}.data

# Clean
tmux kill-window -t "w"

# Execute run
for i in {1..10000}
  do
  $SELF_DIR/run.sh  ${BOARDID} ${CFG_DIR}/minbias.yaml 1235 $SLC_FILE 1236 $DATA_FILE "w" 0
done 
if [ $# == 8 ]
then
  # Log run id
  cp $DATADIR/minbias.yaml  $DATADIR/M$NRUN'_b'$BOARDID.yaml
  rm $DATADIR/last_run.txt
  echo $NRUN >> $DATADIR/last_run.txt
fi

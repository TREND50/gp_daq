#!/bin/sh

SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
PROG_DIR=$SELF_DIR/../target/release/
CFG_DIR=${SELF_DIR}/../cfgs

if [ "x$DATADIR" = x ]
then
    echo "set DATADIR first"
    exit
fi


# Configuration
#BOARDID=$1

if [ -f $DATADIR/last_run.txt ]
then
 :
 else
  echo 0 > $DATADIR/last_run.txt
fi
NRUN=$(<$DATADIR/last_run.txt)
NRUN=$(($NRUN+1))
echo "NRUN=" $NRUN

# Clean
tmux kill-window -t "minbias"  
# Execute run
SLC_FILE=$DATADIR/S${NRUN}.yaml
DATA_FILE=$DATADIR/M${NRUN}.data

cat boardsIn.txt | while read BOARDID
do
  echo "Now doing board " $BOARDID
  $SELF_DIR/run.sh  ${BOARDID} $CFG_DIR/minbias.yaml 1235 $SLC_FILE 1236 $DATA_FILE "minbias" 100
done

# Log run id
cp ${CFG_DIR}/minbias.yaml  $DATADIR/M$NRUN.yaml
rm $DATADIR/last_run.txt
echo $NRUN >> $DATADIR/last_run.txt
echo "Now killing tmux window minbias." 
tmux kill-window -t "minbias"

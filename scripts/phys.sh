#!/bin/sh

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
if [ $# == 8 ]
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
  echo "Usage:" $0 "<board ID> <Th1-> <Th1+> <Th2-> <Th2+> <Th3-> <Th3+>  <TrigPattern> "
  exit 
fi


echo "NRUN=" $NRUN

python $SELF_DIR/fill_phys_yaml.py ${CFG_DIR}/physgen.yaml $2 $3 $4 $5 $6 $7 $8 $DATADIR/phys${NRUN}.yaml


SLC_FILE=$DATADIR/S${NRUN}.yaml
DATA_FILE=$DATADIR/R${NRUN}.data


# Clean
tmux kill-window -t "w"

# Execute run
$SELF_DIR/run.sh  ${BOARDID} $DATADIR/phys${NRUN}.yaml 1235 $SLC_FILE 1236 $DATA_FILE "w" 0

if [ $# == 8 ]
then
  # Log run id
  mv $DATADIR/phys${NRUN}.yaml  $DATADIR/R$NRUN'_b'$BOARDID.yaml
  rm $DATADIR/last_run.txt
  echo $NRUN >> $DATADIR/last_run.txt
fi

#/bin/sh

SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
CFG_DIR=$SELF_DIR/../cfgs
DAQ_DIR=$SELF_DIR/../

if [ "x$DATADIR" = x ]
then
    echo "set DATADIR first"
    exit
fi

# Pattern run
# Parameters: pattern mode

if [ $# != 2 ]
then
    echo "Usage:" $0 "<board ID> <pattern type>"
    exit
fi

# Configuration
BOARDID=$1

#DATADIR=$HOME/GRANDproto/tests/board01/170925_1950
echo $DATADIR
echo "Reading $DATADIR/last_run.txt"

if [ -f $DATADIR/last_run.txt ]
then
:
else
    echo 0 > $DATADIR/last_run.txt
fi

NRUN=$(<$DATADIR/last_run.txt)  # Cannot be executed within a python script.
echo 'Present run ID' $NRUN
NRUN=$(($NRUN+1))
echo 'New run ID' $NRUN

SLC_FILE=$DATADIR/S${NRUN}_b${BOARDID}.yaml
DATA_FILE=$DATADIR/P${NRUN}_b${BOARDID}.data


if [ $2 = "0"  ]
then
  patterncfg=${CFG_DIR}/pattern_zeros.yaml
elif  [ $2 = "1"  ]
then
  patterncfg=${CFG_DIR}/pattern_ones.yaml
elif  [ $2 = "2"  ]
then
  patterncfg=${CFG_DIR}/pattern_toggle.yaml
elif  [ $2 = "3"  ]
then
  patterncfg=${CFG_DIR}/pattern_deskew.yaml
elif  [ $2 = "4"  ]
then
  patterncfg=${CFG_DIR}/pattern_sync.yaml
else  
  echo 'Wrong parameter value. Aborting.'
  exit -1
fi
echo 'Calling ' $patterncfg

# Clean
tmux kill-window -t "w"
# Execute run
$SELF_DIR/run.sh $BOARDID $patterncfg 1235 $SLC_FILE 1236 $DATA_FILE "w" 100
#sleep 2

# Log run id
cp $patterncfg  $DATADIR/P$NRUN'_b'$BOARDID.yaml
rm $DATADIR/last_run.txt
echo $NRUN >> $DATADIR/last_run.txt
echo "Now killing tmux window w." 
tmux kill-window -t "w"

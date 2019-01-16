#!/bin/sh

SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
PROG_DIR=$SELF_DIR/../target/release/
CFG_DIR=${SELF_DIR}/../cfgs

# Configuration
#BOARDID=$1
NRUN=$(<$DATADIR/last_run.txt)
NRUN=$(($NRUN+1))

# Clean
tmux kill-window -t "minbias"  

${SELF_DIR}/run_server.sh 1235 1236 $DATADIR/M$NRUN.data "minbias"

cat boardsIn.txt | while read BOARDID
do
  echo "Now doing board " $BOARDID
  for i in {1..500}
  do 
    echo "Doing event " $i
    ${PROG_DIR}/send_msg ${CFG_DIR}/minbias.yaml 192.168.1.1${BOARDID}:1234 8888
  done
  #sleep 1
  #$SELF_DIR/run.sh  1235 1236 192.168.1.1${BOARDID} ${CFG_DIR}/slcreq.yaml $DATADIR/S$NRUN'_b'$BOARDID.data "wslc" 0
  # Get slc data as well
  #${PROG_DIR}/send_msg ${CFG_DIR}/slcreq.yaml 192.168.1.1${BOARDID}:1234 8888
done

#BOARDID=09
#$SELF_DIR/run.sh  1236 192.168.1.1${BOARDID} minbias.cfg $DATADIR/M$NRUN'_b'$BOARDID.data "wmb" 500
#BOARDID=10
#$SELF_DIR/run.sh  1236 192.168.1.1${BOARDID} minbias.cfg $DATADIR/M$NRUN'_b'$BOARDID.data "wmb" 500
#BOARDID=11
#$SELF_DIR/run.sh  1236 192.168.1.1${BOARDID} minbias.cfg $DATADIR/M$NRUN'_b'$BOARDID.data "wmb" 500
#BOARDID=25
#$SELF_DIR/run.sh  1236 192.168.1.1${BOARDID} minbias.cfg $DATADIR/M$NRUN'_b'$BOARDID.data "wmb" 500
#BOARDID=27
#$SELF_DIR/run.sh  1236 192.168.1.1${BOARDID} minbias.cfg $DATADIR/M$NRUN'_b'$BOARDID.data "wmb" 500
#BOARDID=31
#$SELF_DIR/run.sh  1236 192.168.1.1${BOARDID} minbias.cfg $DATADIR/M$NRUN'_b'$BOARDID.data "wmb" 50


#for i in {1..500}
#do 
#  echo 'Event' $i
#  # Execute run
#  $SELF_DIR/run.sh  1236 192.168.1.1${BOARDID} minbias.cfg $DATADIR/M$NRUN'_b'$BOARDID.data "wmb" 0
#done

# Log run id
cp ${CFG_DIR}/minbias.yaml  $DATADIR/M$NRUN'_b'$BOARDID.yaml
rm $DATADIR/last_run.txt
echo $NRUN >> $DATADIR/last_run.txt
echo "Now killing tmux window minbias." 
tmux kill-window -t "minbias"

#! /bin/sh

# THis is a script we use to first set GPS mode, wait for lock and then start run. This requires that ant_on = 0 in physgen.yaml
SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
CFG_DIR=$SELF_DIR/../cfgs

./loopStop.sh # Stopp all present acquisitions

cd ..
 # sudo target/release/set_addr enp3s0f1 ./scripts/addr.yaml 1234 888
cd scripts

if [ -f $DATADIR/last_run.txt ]
then
:
else
  echo 0 > $DATADIR/last_run.txt
fi
NRUN=$(<$DATADIR/last_run.txt)
NRUN=$(($NRUN+1))

if [ $# != 0 ]
then
    echo $1 > thisBoardsIn.txt
else
    cp boardsIn.txt thisBoardsIn.txt
fi

thl=100
thh=100
# Start run on all antennas
cat thisBoardsIn.txt | while read BOARDID
do
  echo ""
  echo "*** Now setting physics run on board" $BOARDID
  if [ $BOARDID = "03" ] || [ $BOARDID = "05" ] || [ $BOARDID = "09" ] || [ $BOARDID = "18" ] || [ $BOARDID = "19" ] || [ $BOARDID = "16" ] || [ $BOARDID = "06" ]; then
    ./phys.sh $BOARDID $thl $thl $thl $thl $thl $thl 011000 $NRUN  #Warning: ant_on = 0 in phys.sh
  else
    ./phys.sh $BOARDID $thh $thh $thh $thh $thh $thh 011000 $NRUN  #Warning: ant_on = 0 in phys.sh 
  fi
  cargo run --bin send_msg --release $CFG_DIR/gps.yaml 192.168.1.1$BOARDID:1234 8888  # Set GPS fixed mode
done

echo "Now waiting 60s to lock GPS position..."
#sleep 60  # Wait 10 minutes
SLC_FILE=$DATADIR/S${NRUN}.yaml
DATA_FILE=$DATADIR/R${NRUN}.data

# Start run on all antennas
cat boardsIn.txt | while read BOARDID
do
  # Execute run
  echo "*** Now starting physics run on board" $BOARDID
  ./run.sh  ${BOARDID} $CFG_DIR/reset_DAQ.yaml 1235 $SLC_FILE 1236 $DATA_FILE "w" 0 # Now set ant_on = 1 and actually start DAQ
done

rm $DATADIR/last_run.txt
echo $NRUN >> $DATADIR/last_run.txt


  
for i in {1..100000}  # Infinite loop
  do
  cat thisBoardsIn.txt | while read BOARDID
  do
    ./slcreq.sh $BOARDID 0
  done
  sleep 10
done


# Useless because this happens after infinte time
#echo "Now killing data tmux window w." 
#tmux kill-window -t "w"


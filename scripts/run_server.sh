#!/bin/sh

SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
PROG_DIR=$SELF_DIR/../target/release/

if [ $# != 4 ]
then
    echo "Usage:" $0 " <slc port> <data port> <dump file> <session name>"
    exit
fi



SLC_PORT=$1
DATA_PORT=$2
DUMP_FILE=$3
SESSION_NAME=$4

session_exists=0

if tmux ls 2>/dev/null
then
    for i in `tmux ls|awk -F ':' '{print $1}'`
    do
		if [ $i == $SESSION_NAME ]
		then
			session_exists=1
			#echo "Session exists."
			break
		fi
    done
fi

if [ $session_exists == 0 ]
then
#        echo "Starting session."
	tmux new -d -s $SESSION_NAME
	sleep 0.1
fi

tmux select-pane -t 0
echo "Now starting server."
tmux send-keys "$PROG_DIR/trend_server 0.0.0.0 ${SLC_PORT} ${DATA_PORT} 8888 $DUMP_FILE" C-m
#sleep .5  # Needed on laptop

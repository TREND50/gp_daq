#/bin/sh

SELF_PATH=`realpath $0`
SELF_DIR=`dirname $SELF_PATH`
CFG_DIR=$SELF_DIR/../cfgs
DAQ_DIR=$SELF_DIR/../
SELF_DIR=`dirname $SELF_PATH`
PROG_DIR=$SELF_DIR/../
CFG_DIR=$SELF_DIR/../cfgs

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


# Configuration
BOARDID=$1


# Clean
tmux kill-window -t "w"
# Execute run
cargo run -q --manifest-path $PROG_DIR/Cargo.toml --bin send_msg -- ${CFG_DIR}/stop.yaml 192.168.1.1${BOARDID}:1234 8888

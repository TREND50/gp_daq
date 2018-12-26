# Start the Data Acquisition

## Run the DAQ from scratch 
In this section, we describe how to run each single command for DAQ

### Start a server

#### The concept of ```server```
A ```server``` is a process (or in other words, a program) listening to a certain SLC port and a certain payload data port, receiving corresponding  messages and storing the messages to disk.

A server process also response to any Ack message from DAQ boards. 
It will forward Ack to a local monitor port (127.0.0.1:<Monitor Port>), so that the sending message process will be informed about whether remote board has received the command.
The idea is that the sending command process (usually ```send_msg```) will start its own mini-server (do not confused with the previously mentioned server process) thread, which listens to the monitor port. 
As soon as the server process receives any Ack message from one DAQ board, it will be forwarded to the mini-server thread through the monitor port. 
Then the sending message process can get known that the remote board has received and responsed the command that it has just sent.


#### The command to start the server
```
cargo run --bin trend_server --release 0.0.0.0 $SLC_PORT $DATA_PORT $MON_PORT $OUT_PREFIX
```
where ```$SLC_PORT``` is the port for receiving the SLC msg, which is usually 1235
```$DATA_PORT``` is the port for receiving the DATA msg, which is usually 1236
```$MON_PORT``` is the port that used to forward the Ack message, which is arbitrary, and the suggested value is 8888.
```OUT_PREFIX``` is the prefix of output files. TRENDDATA message will be save to ```${OUT_PREFIX}.bin``` and all other messages, but Ack msg will be saved to ```${OUT_PREFIX}.bin```, and Ack msg will only be forwarded.
The command line argument ```0.0.0.0``` instructs the server to bind ports to all local ip addresses.
In current stage, TRENDDATA will also be save to ```${OUT_PREFIX}.yaml``` file for debugging.
Later this feature will be removed.

### Send a command to remote board in order to do some operation
The command to operate remote DAQ boards is sent through the program ```send_msg```.
The commands are defined in YAML files. 
Each YAML file can contain more than one message, and they are sent one by one.
Running following command will cause messages be sent to one desired board.
```
cargo run --bin send_msg --release some.yaml ${BOARD_IP}:${BOARD_PORT} ${MON_PORT}
```
The arguments are self-explained.


## Operate DAQ boards with scripts
The scripts for operating DAQ boards are merely batched commands.

### ```run.sh```
The most foundational script is the ```scripts/run.sh```
It can be called with following arguments
```
scripts/run.sh $SLC_PORT $DATA_PORT $BOARD_IP some.yaml $SESSION_NAME $LOOP
``` 
This script will start a ```tmux``` session, in which a ```trend_server``` is runed.
After the background ```trend_server``` is started, this script will call the ```send_msg``` command to send commands defined in the file some.yaml.
```$LOOP``` argument defines how many times the command should be sent. Other arguments should be self-explained.

### Other scripts
Currently only two scripts have been verified they are ```phys.sh``` and ```minBias.sh```
Both scripts call run.sh to do the actual jobs.
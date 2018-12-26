# Start the Data Acquisition

## Run the DAQ from scratch 
In this section, we describe how to a GRAND DAQ Master within few minutes :-)


### Communication between central DAQ and Front-End Units
GRANDProto35 DAQ has a multi-layer structure. At the core of it is a very basic and rigid system of formated words (see details here: xxx) exchanged between the Front-End UNits (called FEUs in the following) and the central DAQ program (cDAQ). The protocol used for comunication is UDP, fast but not secured (i.e. no built-in process to ensure that a package sent was received).

Two main processes are in charge of communication on the cDAQ side:
- ```send_msg``` is its "mouth". It sends commands to operate remote FEUs in the form of the above-mentionned words. 
- the ```trend_server``` process is its "ears". It listens to a pre-defined SLC port and a certain payload data port (see 
        gp_daq/doc/setting_addr.md
       for details). It receives messages from the remote FEUs connected to these ports, and stores these messages to disk (see below for details).

Note here that there is no direct communication between ```send_msg``` and ```trend_server```. However, when a FEU succesfully receives a command, it sends back an acknowledgement message (noted ACK in the following) which is thus read by the ```trend_server```. ```trend_server``` then forwards the ACK message to a local monitor port (127.0.0.1:<Monitor Port>). ```send_msg``` will for its part start its own mini-server (not to be confused with the previously mentioned ```server```) thread, which listens to this very same monitor port. This workaround allows to garantee that the initial command was succesfully received.
  
(The trick here is that ```send_msg``` will start its own mini-server (not to be confused with ```trend_server```) thread, which listens to the monitor port. As soon as the server process receives any ACK message from one FEU, it will forward it to the mini-server thread through the monitor port and the ```send_msg``` gets the information.)

### Sending commands

Standard commands have been written in files in YAML format. We recommand to use them only. The ```send_msg``` will read these files, interpret them and build teh corresponding formated words to be sent to the FEUs through this command:
```
cargo run --bin send_msg --release some.yaml ${BOARD_IP}:${BOARD_PORT} ${MON_PORT}
```
where ```$BOARD_IP``` is the IP adress of the targetted FEU, ```$BOARD_PORT``` the port that used to send messages (usually 1234) and```$MON_PORT``` the Monitor Port (arbitrary value set in 
        gp_daq/doc/setting_addr.md
      , 8888 is suggested).


### Running ```trend_server```

The ```trend_server``` process is run the following way:

```
cargo run --bin trend_server --release 0.0.0.0 $SLC_PORT $DATA_PORT $MON_PORT $OUT_PREFIX
```
where ```$SLC_PORT``` is the port for receiving the SLC msg (usually 1235)
```$DATA_PORT``` is the port for receiving the DATA msg, (usualy 1236)
```$MON_PORT``` is the port that used to forward the Ack message (arbitrary value set in , 8888 is suggested).
```OUT_PREFIX``` is the prefix of output files. 

TRENDDATA messages will be saved to ```${OUT_PREFIX}.bin```, ACK msg will only be forwarded (see above) and all other received messages saved to ```${OUT_PREFIX}.txt```.
The command line argument ```0.0.0.0``` instructs the ```trend_server``` to bind ports to all local IP addresses.
In current stage, TRENDDATA will also be saved to (human readable) ```${OUT_PREFIX}.yaml``` files for debugging.
Later this feature will be removed to save some disk space and bandwidth.



## Operate DAQ boards with scripts
Scripts were written to operate DAQ boards. They are merely batched commands.

### ```run.sh```
The most foundational script is ```scripts/run.sh```
It can be called with following arguments
```
scripts/run.sh $SLC_PORT $DATA_PORT $BOARD_IP some.yaml $SESSION_NAME $LOOP
``` 
This script will start a ```tmux``` session, in which a ```trend_server``` is runed.
After the background ```trend_server``` is started, this script will call the ```send_msg``` command to send commands defined in the file some.yaml.
```$LOOP``` argument defines how many times the command should be sent. Other arguments should be self-explained.

### Other scripts
Currently only two scripts have been verified: ```phys.sh``` and ```minBias.sh```. ```phys.sh``` allows to start a "standard" acquisition with triggers from the X & Y channels of each antenna. ```minBias.sh``` collects soft-triggered samples of data ---minimal-biased, hence the name of script--- which can be used to monitor the quality of the data taking. The variation of the baseline level of the signal with time in particular allows to measure the ~ daily fluctuation expected from teh transit of the galactic plane in the antenna field of view.


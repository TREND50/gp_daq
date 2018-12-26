# Initiallizing ip addresses

## Relevant programms:
1. set_addr
2. trend_server


## Steps
1. Go to directory ```scripts/```
2. Ensure there is a txt file containing two columns with 1st column to be the mac address and the 2nd column to be the subnet IP address. At present this file is called ```mac_file.txt```. In the following, we will assume that the name of the file is $MAC_FILE.
3. Confirm the name of the network interface card usually using```ifconfig```. It should be the card associated with the IP adress 192.168.1.10. At present it is ```enp3s0f1```, but in other cases it could be ```eth0```. In the following, we will assume that the name of the interface card is $IFACE
4. Run the following commands:
```
$> cd scripts/
$> ./gen_ip_cfg.sh $IFACE $MAC_FILE >addr.yaml
$> cd ../
#Optionally run a server to receive the ACK
$> cargo run --bin trend_server --release 0.0.0.0 1235 1236 8888 some_name_not_important
$> sudo cargo run --bin $IFACE --release scripts/addr.yaml 1234 8888
```

where ```8888``` is the monitoring port, its value is arbitrary.

After running the above commands, the board should be succesfully initialized. If this is the case, then an ACK msg will be sent back in response to any command from the central DAQ and forwarded to the set_addr program.

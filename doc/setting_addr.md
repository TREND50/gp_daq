# Initiallizing ip addresses

## Relavent programms:
1. set_addr
2. trend_server


## Steps
1. cd to ```scripts/```
2. Ensure there is a file containing two columns with 1st column to be the mac address and the 2nd column to be the subnet IP address, defaultly there should be one. In following steps, we assume the name of the file to be $MAC_FILE
3. Confirm the name of the network interface card usually something like ```eth0```, ```enp3s0f1```. In following steps, we assume the name of the interface card to be $IFACE
4. Run following command:
```
$> cd scripts/
$> ./gen_ip_cfg.sh $IFACE $MAC_FILE >addr.yaml
$> cd ../
#Optionally run a server to receive the ACK
$> cargo run --bin trend_server --release 0.0.0.0 1235 1236 8888
$> sudo target/release/set_addr enp3s0f1 --release scripts/addr.yaml 1234 8888
```

where ```8888``` is the monitoring port, its value is arbitrary.

After running above commands, if any board has been initialized to its desired IP address, then ACK msg will be responsed by the server program and forwarded to the set_addr program, so that you can check whether the board has any response.

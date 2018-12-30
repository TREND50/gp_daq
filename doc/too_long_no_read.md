# All following commands should be executed in ```gp_daq``` directory.

# Setting the IP addresses before running DAQ
Assuming the name of the network interface card corresponding to IP 192.168.1.10 is ```enp3s0f1```(check it with the command ```ifconfig```), then do:

```
$> cd scripts/
$> ./gen_ip_cfg.sh enp3s0f1 mac_file.txt >addr.yaml  # generte the up-to-date yaml file to set the adresses
$> cd ../
$> cargo run --bin trend_server --release 0.0.0.0 1235 1236 8888 some_name_not_important  # start the trend_server
$> sudo cargo run --bin set_addr enp3s0f1 --release scripts/addr.yaml 1234 8888  # set the adresses
```

# Start a server
```
cargo run --bin trend_server --release 0.0.0.0 1235 1236 8888 /mnt/data/asdf
```

# Send command
```
cargo run --bin send_msg --release some.yaml 192.168.1.105:1234 8888
```
This will send the message encoded in file ```some.yaml``` to the Front-End unit with ID 05.

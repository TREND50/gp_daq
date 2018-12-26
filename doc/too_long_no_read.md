# All following commands should be executed in ```gp_daq``` directory.

# Setting the ip addresses before DAQ
Assume the name of the network interface card that connects to the DAQ boards is ```enp3s0f1```(check it with the command ```ifconfig```

```
$> cd scripts/
$> ./gen_ip_cfg.sh enp3s0f1 mac_file.txt >addr.yaml
$> cd ../
$> cargo run --bin trend_server --release 0.0.0.0 1235 1236 8888 some_name_not_important
$> sudo cargo run --bin enp3s0f1 --release scripts/addr.yaml 1234 8888
```

# Start a server
```
cargo run --bin trend_server --release 0.0.0.0 1235 1236 8888 /mnt/data/asdf
```

# Send command
```
cargo run --bin send_msg --release some.yaml 192.168.1.105:1234 8888
```

# All following commands should be executed in ```gp_daq``` directory.

# Start a server
```
cargo run --bin trend_server --release 0.0.0.0 1235 1236 8888 /mnt/data/asdf
```

# Send command
```
cargo run --bin send_msg --release some.yaml 192.168.1.105:1234 8888
```

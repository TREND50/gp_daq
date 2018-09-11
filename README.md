# Grandproto Data Acquisition program

## Why uspgrading from the original program?
1. The [original program](https://github.com/TREND50/GRANDproto_DAQ)
was written in C++, which has much less modern
features compared with the currently using
[RUST](http://www.rust-lang.org) language.
2. The original version was finished in a very short time, and not well
 designed, so that a fully refactored one can correct the mistakes in the previous version.

## Compatibility
1. There is no change in communication protocol
2. The text form description of the message changed from the original
private format to YAML format, which makes it more universal and more
easy to be used in other programs.
3. Originally used Eventfile is still the the binary format used to
store ADC data.
4. The calling arguments are approximately same as the original version,
 except that the address and port should be given in the form
  ```addr:port```, for example ```192.168.1.118:1234``` for target
  address and ```0.0.0.0:1234``` for server address and port binding.

## Installation
1. Install ```RUST``` with [rustup](http://rustup.rs)

simply run command:
```
curl https://sh.rustup.rs -sSf | sh

```
Then use follow the instruction.

After the installation is finished, RUST compiler stable version is
installed, but some feature need nightly version. So we have to switch
the RUST from stable to nightly.

This is done by
```
rustc default nightly
rustup toolchain remove stable
```

In order to format the source code files (to make them look pretty),
let's install the rustfmt:
```
cargo install rustfmt-nightly
```

2. compile the program

Clone this respository and change directory into the project root path
and run the compilation command
```
cd gp_daq
cargo build --release
```

If necessary, you can install it into your user rust binary directory by
```
cargo install --force
```

## Run the program
### To install or not to install?
If you decide not to install the binary files, you need to run the
command as
```
cargo run --release --bin <command> <args>
```

If you decide to install them you can omit ```cargo run --release --bin```
and directly run the command as
```
<command> <args>
```

We simply assume you are going to install the program in following
description.

### Commands for operating GRANDproto DAQ system
1. Send message to DAQ board

```
send_msg example_cfg/pattern.yaml <ip>:<daq port> <monitor port>
```
, where <daq port> is the port of the DAQ system that is set beforehand
for listening the operating messages for example ```1234```,
<monitor port> is an arbitrary port for the command ```send_msg```
to receive ```ACK``` message from the DAQ board (well, actually the
```ACK``` message replied by the DAQ board will be received by the
```trend_server``` process and forwarded to address
```127.0.0.1:<monitor port>```. The content of the message to be sent
is written in a ```YAML``` file, which is basically a key-value table.

2. Run the data receiving server
```
trend_server 0.0.0.0:<port> <monitor port> [data file prefix]
```
, where <port> is the port used to receive message from the DAQ boards,
and <monitor port> is used to forward ACK messages to ```send_msg```
program to check whether the board responses to the sent controlling
messages.



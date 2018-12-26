# Installation

## Install ```RUST``` with [rustup](http://rustup.rs)

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

## compile the program

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

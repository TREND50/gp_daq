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

## Check [doc](doc) for other information

## ToDo
- Solve DAQ stalling issue (see eg R238)
- Remove "Ack" messages in SLC data.
- Check and clean "NoAck" information.
- Implement different trigger levels for different channels and different detection units in [loopPhys.sh](https://github.com/TREND50/gp_daq/blob/master/scripts/loopPhys.sh)
- More generaly clean up & optimise scripts.
- Implement Calibration script. May be based on (obsolete) script [gp_DAQ/calib.sh](https://github.com/TREND50/GRANDproto_DAQ/blob/master/calib.sh)

- Longer term: implement online coincidence search.
- Test DAQ for larger number of antennas.

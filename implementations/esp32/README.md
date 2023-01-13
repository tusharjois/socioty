# socioty-esp32
socioty for esp32 boards


## Setup

Install [`espup`](https://esp-rs.github.io/book/installation/installation.html#espup):

```sh
cargo install espup
espup install
```

For the simulator, install [`wokwi-server`](https://github.com/MabezDev/wokwi-server), which launches an ESP-32 in a web simulator:

```sh
cargo install wokwi-server --git https://github.com/MabezDev/wokwi-server --locked
```

Make sure that the `socioty` repository is in the folder `../socioty`, relative to this repository.

## Building

Before every invocation of Cargo, you need to source `~/export-esp.sh` to load environment variables:

```sh
. ~/export-esp.sh
```

Then, you can build the package:

```sh
# Assuming you're already in implementations/esp32
cargo build # Add --release for benchmarks
```

## Running in the simulator

Once the build is ready, you can run it in the simulator:

```sh
# Still in implementations/esp32
wokwi-server --chip esp32 ../../target/xtensa-esp32-espidf/debug/socioty_esp32_eval # or whatever you're testing
```

This launches a web browser that connects to the server and runs the machine. I had trouble getting it to work in Safari, so i just paste the link into Firefox and it works fine.

## Running benchmarking for esp32
usage: esp_benchmarking <logfile directory> <init> <reconstruct> <eval>
- *init* run init test
- *reconstruct* run reconstruct testing
- *eval* run eval testing

#### Hardcoded values that need to be changed
- ```export_sh="/Users/logan/export-esp.sh"``` location of export-esp.sh script
- Device names:
    - ```device1="usbserial-144460"``` 
    - ```device2="usbserial-1444710"```
    - ```device3="usbserial-1444720"```
    - ```device4="usbserial-1444730"```
    - ```device5="usbserial-1444740"```

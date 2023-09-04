# Artifact Appendix

Paper title: **SocIoTy: Practical Cryptography in Smart Home Contexts**

Artifacts HotCRP Id: **#15**

Requested Badge: **Reproducible**

## Description
This artifact is the source code repository used for SocIoTy, an at-home cryptographic system. We provide instructions for the `microbenchmark`, `coap`, and `esp32` parts of the evaluation.

### Security/Privacy Issues and Ethical Concerns
N/A

## Basic Requirements

### Hardware Requirements
- `microbenchmark` testing: One Raspberry 2/3/Zero
- `coap` testing: Minimum of three RPIs 2/3/Zero (any combination)
- `esp32` testing: 1 ESP32 microcontroller board

### Software Requirements
Docker.

### Estimated Time and Storage Consumption
The Docker image is ~5GB. The container requires ~4GB of system memory.

Time estimates: Really based on if you use Rapsberry Pi Zeros in the testing, since its hardware is slower; Pi 2s and Pi 3s run faster. The scripts do provide updates as benchmarks progess.
- `microbenchmark` testing: Can take up to a couple hours of testing, depending on the number of iterations selected. Our work ran 100k iterations. One RPI3 took about 13 minutes for 1000 iterations.
- `coap` testing:  Dependent on the number of nodes, with 4 nodes and 100 iterations, took about 5 minutes. (Hardcoded to run 1000 iterations at the moment).

## Environment

### Accessibility
- GitHub Repository: https://github.com/tusharjois/socioty/
- Commit Branch: `artifact_review`

### Set up the environment

There are three parts of the environment. The first consists of all of the Raspberry Pi devices used as nodes for the SocIoTy system. The second is a coordinating node that handles the running of each benchmark. This coordinating node is run inside of a Docker container. The final part of the environment is the ESP32 toolchain, which is run on the machine with USB access to an ESP32 device.

#### Raspberry Pi Setup

Each Raspberry Pi needs to be configured as follows:

- A `node` user with a home directory "/home/node" and the following directories
    - `/home/node/benchmarks/`
    - `/home/node/benchmarks/output/`
    - `/home/node/coap/`
- An SSH authorized keys file `/home/node/.ssh/authorized_keys` file
- Connected to the same network

#### Docker container setup

On the coordinating node, perform the following steps:

1. Install [Docker](https://docs.docker.com/engine/install/) for your platform.

2. Clone the repository for the artifact.

```bash
git clone https://github.com/tusharjois/socioty.git
cd socioty/
git checkout artifact_review
```

3. Add the IP addresses of each Raspberry Pi to the `ssh_pi.configs` file. **Note**: RPi 2/3 and RPi Zero IP addresses are seperated intentionally. Make sure to add the IP address to the appropriate list. *Failure to do this properly will cause the testing to fail.*

4. Build the Docker container.
 
```bash
docker build -t socioty .
docker run --entrypoint /bin/bash -itd socioty
```

5. Access the container and run the following command to retreive the SSH key of the Docker container.

```bash
cat /root/.ssh/socioty_nodes.pub
```

6. Add the public SSH key from the Docker container to the `/home/node/.ssh/authorized_keys` file for **each** node.

#### ESP32 Setup
On the system with ESP32 access, perform the following steps:

```bash
# Install Rust
curl https://sh.rustup.rs -sSf | bash -s -- -y
PATH="/root/.cargo/bin:${PATH}"

# Install espup, the ESP32 rust toolchain
cargo install espup
espup install

# Source the environment from the espup installer
. ~/export-esp.sh

# Build the ESP32 binary
cd implementations/esp32/
cargo build --release
```

**Note**: The toolchain for building for ESP32 targets is messy. We have included the ESP32 binary in this GitHub repo if there are 
issues building it locally.

Before running the experiments, the following variables in the `implementations/esp32/esp_benchmarking.sh` need to be updated:

- `export_sh` this variable is the location of the "export-esp.sh" script
- `device<n>` the device variables are the ESP32 boards connected to your machine. `/dev/tty.xxxx` is the typical
location. Commented out variables are shown for an example.

### Testing the Environment
The Docker container will fail to build if there is an issue with the first two parts of the environment.
The ESP32 toolchain will return an error is there is an issue with the third part of the environment.

## Artifact Evaluation

### Main Results and Claims

#### Main Result 1: Microbenchmark execution time on Raspberry Pis
The first result is the microbenchmarks for Gen, PartialEval, PartialEval (AE), and Recon running on each RPi. These results are found in Section 5.1, in Table 2 and Figures 6 & 7.

#### Main Result 2: CoAP execution time on Raspberry Pis
The next result is the scalability benchmarks using CoAP for the entire SocIoTy system. These results are found in Section 5.2 in Figure 8 and in the Appendix in Table 6.

#### Main Result 3: Microbenchmark execution time on ESP32
The final result we claim as a part of this artifact is the microbenchmark results for the ESP32. These results are found in Section 5.1 in Table 2 and in the Appendix in Tables 4 & 5.

### Experiments
For each experiment, the time can range from 20 minutes to a couple hours. The `parse_data.py` script output should
align with the listed figures and tables found in the paper. Since this is operating over network and on potentially different devices,
times may vary slightly.

#### Experiment 1: Raspberry Pi Microbenchmarks
This experiment is to generate the microbenchmark data for the RPi. Once the docker container and RPIs are configured, run

```bash
cd benches/
./microbenchmark_deployment no send test <# of iterations> retrieve 
```

This will send the binaries to the nodes, run the microbenchmarks, and retrieve the results file. The number of iterations option will greatly change the 
run time. On a RPI3, it took about 13 minutes for the tests to complete at 1000 iterations. After the tests finish running, the data will be populated in the
`microbenchmark_data` directory.

You can run the `parse_data.py` script to generate the tables. The following variables need to be updated:
- Line 9: `iterations` the number of iterations ran for the previous step
- Line 47: `nodes` this dict should align to the experimental setup

After these steps:

```bash
cd benches/
python3 parse_data.py --bench eval --all microbenchmark_data latex_table
python3 parse_data.py --bench init --all microbenchmark_data latex_table
python3 parse_data.py --bench reconstruct --all microbenchmark_data latex_table
```

#### Experiment 2: Raspberry Pi CoAP Benchmarks
This experiment is to generate the microbenchmark data for the CoAP runs on the RPis. On the docker container and RPis are configured, run:

```bash
cd benches/
./coap_benchmarking
```

This will send the required binaries to the nodes, start the CoAP servers on the nodes, run the requests, output results to a file. This will loop over the 
number of nodes, n, from n-2 to n.

After the tests finish running, the data for each run is located in the `coap_data` directory with the results in a file called `run_$n_$t_output.txt`.

You can run the `parse_data.py` script to generate the tables.

```bash
cd benches/
python3 parse_data.py --all coap_data latex_table
```

#### Experiment 3: ESP32 Microbenchmarks
This experiment is to generate the data for ESP32 microbenchmark execution times.

After configuring the script variables (above) run the following:
```bash
cd implementations/esp32/
./esp_benchmarking.sh <socioty>/benches/esp32_data/ init reconstruct eval
```

You can run the `parse_data.py` script to generate the tables. The following variable needs to be updated:

- The `serial_ids` variable (line 98, `parse_data.py`) needs to be updated with the serial devices that were used to run the devices. 

After these steps:

```bash
cd benches/
python3 parse_data.py --bench init esp32_data latex_table
python3 parse_data.py --bench eval esp32_data latex_table
python3 parse_data.py --bench reconstruct esp32_data latex_table
```

## Limitations
The full end-to-end deployment (Figure 9) was not included in this artifact due to the complexity of reproducing the setup for ESP32 MQTT nodes and the iOS MQTT app. To aid future research, however, we include our C foreign-function interface code (`implementations/ffi`) and our MQTT server code (`implementations/mqtt`) for review. These implementations were used in the development of the end-to-end deployment, and will help others with their deployments on different hardware types.

## Notes on Reusability

The Rust implementations of the TDPRF under `src/` are resuable and generic to be used on other devices as long as there is a Rust target. We believe the additional implementations under `implementations/` will also aid in the deployment of SocIoTy under different smart home architectures and device hardware.

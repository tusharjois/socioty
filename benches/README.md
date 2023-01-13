# Scripts
## Setup
1. Refer to .cargo/config.toml for target installation
2. Each node has been configured in .ssh/config as <node$i> where 1 <= i <= 12 
## Host contained scripts
### microbenchmark_deployment
usage: microbenchmark_deployment <build> <send>
- Goal: compile and deploy the target builds to the nodes
- *build* compiles both rpi 2/3 and rpi zero targets
- *send* sends the targts to the corresponding nodes
- note: uses colored output, could remove if script fails to load the module

### coap_benchmarking
usage: coap_benchmarking <configs> <run>
- Goal: Run coap benchmarking across the nodes. Host machine runs coap request and kills processes on the nodes through cleanup_procs script.
- *configs* generates config files for each node
- *run* Send configs files to the nodes, runs the test

### mqtt_benchmarking
usage: mqtt_benchmarking.sh <configs> <run>
- Goal: Run matt benchmarking across the nodes. Host machine runs coap request and kills processes on the nodes through cleanup_procs script.
- *configs* generates config files for each node
- *run* Send configs files to the nodes, runs the test

## Node contained scripts
### local_benchmarking
usage: local_benchmarking <number of rounds>
- Goal: Run socioty_benchmark_init and socioty_benchmark_reconstruct for *x* number of rounds
### cleanup_procs
usage: cleanup_procs <proc to kill>
- Goal: Run to kill coap node process 
- *proc to kill* process to kill on the node

# Data
## Microbenchmarks

Run date 📅: 24 Jan 2023 - 26 Jan 2023

‼️ Node11 has some crashes in the middle of its runs...

Under each node directory the files are of each test run with the computed run time. 
i.e. reconstruct_i_n_t.txt is the reconstruction with n nodes, threshold of t run i times.

## CoAP
Run date 📅: 29 Jan 2023

This data is a bit messy. Each file is the run_n_t_output.txt same as above. Inside each file has the iteration run times, the line with the duration look like so
```python
Duration: <time> (in milliseconds)
```
Simple grep and awk will work 
```bash
grep "Duration:" <file> | awk '{sum += $2} END {print sum/NR}'
```

## ESP32 Microbenchmarks
Run date 📅: 19 Feb 2023 - 20 Feb 2023

Each test run is named <test_type>_<device>_run_<n>_<t>.txt where n is number of nodes and t is the threshold.
**eval** and **init** output files contain the average on lines
```python
Average time <time>ms
```
**reconstruct** output files contain the average on lines
```python
Average reconstruct time <time>s
```

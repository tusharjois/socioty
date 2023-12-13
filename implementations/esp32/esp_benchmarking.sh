#!/bin/bash
#################################################################
# This script runs the following benchmarks on the ESP32 device #
# socioty_esp32_init for clients=5 to 12 with a                 #
#   threshold of k=clients-2 to clients 1000 runs               #
# socioty_esp32_reconstruct for clients=5 to 12 with a          #
#   threshold of k=clients-2 to clients for 1000 runs           #
# socioty_esp32_eval for 1000 runs                              #
#                                                               #
# Usage: esp_benchmarking.sh  <output dir>                      #
#                                                               #
# Notes: The following variables are hardcoded + RUNS=1000,     #
# those need to be modifed as necessary                         #
#################################################################

export_sh="export-esp.sh"
#export_sh="/Users/node/export-esp.sh"
device1="ttyUSB0"
device2="ttyUSB1"
device3=""
device4=""
device5=""
#device1="tty.usbserial-141410"
#device2="tty.usbserial-1444710"
#device3="tty.usbserial-1444720"
#device4="tty.usbserial-1444730"
#device5="tty.usbserial-1444740"
runs=1000

# Running socioty_esp32_init sequence
if [[ $2 == "init" ]]; then
    for n in {5..12}
    do
        for (( t=$n-2; t<=$n; t++ ))
        do
          if [[ "${device1}" ]]; then
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device1}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device1} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          if [[ "${device2}" ]]; then
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device2}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device2} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          if [[ "${device3}" ]]; then
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device3}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device3} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          if [[ "${device4}" ]]; then
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device4}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device4} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          if [[ "${device5}" ]]; then
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device5}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device5} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          wait
        done
    done
    echo "Done init"
fi
# Running socioty_esp32_reconstruct sequence
if [[ $3 == "reconstruct" ]]; then
    for n in {5..12}
    do
        for (( t=$n-2; t<=$n; t++ ))
        do
          if [[ "${device1}" ]]; then
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device1}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device1} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          if [[ "${device2}" ]]; then
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device2}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device2} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          if [[ "${device3}" ]]; then
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device3}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device3} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          if [[ "${device4}" ]]; then
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device4}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device4} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          if [[ "${device5}" ]]; then
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device5}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/${device5} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
          fi
          wait
        done
    done
    echo "Done reconstruct"
fi
eval_command () {
    eval_command_string="set timeout 600
      log_user 0 
      set logfile \"$1/eval_${2}${3}.txt\"
      log_file -a \$logfile
      spawn bash -c \"source ${export_sh}; RUNS=${runs} cargo espflash --monitor /dev/${2} --bin socioty_esp32_eval ${4} --release\"
      expect \"Fin.\" {send \x03}"
}
# Running socioty_esp32_eval sequence
if [[ $4 == "eval" ]]; then
    if [[ "${device1}" ]]; then
      eval_command "${1}" "${device1}" "" "--features aead"
      expect -c "${eval_command_string}" &
    fi
    if [[ "${device2}" ]]; then
      eval_command "${1}" "${device2}" "" "--features aead"
      expect -c "${eval_command_string}" &
    fi
    if [[ "${device3}" ]]; then
      eval_command "${1}" "${device3}" "" "--features aead"
      expect -c "${eval_command_string}" &
    fi
    if [[ "${device4}" ]]; then
      eval_command "${1}" "${device4}" "" "--features aead"
      expect -c "${eval_command_string}" &
    fi
    if [[ "${device5}" ]]; then
      eval_command "${1}" "${device5}" "" "--features aead"
      expect -c "${eval_command_string}" &
    fi
    wait
    echo "Done eval aead"
    if [[ "${device1}" ]]; then
      eval_command "${1}" "${device1}" "_nomac" ""
      expect -c "${eval_command_string}" &
    fi
    if [[ "${device2}" ]]; then
      eval_command "${1}" "${device2}" "_nomac" ""
      expect -c "${eval_command_string}" &
    fi
    if [[ "${device3}" ]]; then
      eval_command "${1}" "${device3}" "_nomac" ""
      expect -c "${eval_command_string}" &
    fi
    if [[ "${device4}" ]]; then
      eval_command "${1}" "${device4}" "_nomac" ""
      expect -c "${eval_command_string}" &
    fi
    if [[ "${device5}" ]]; then
      eval_command "${1}" "${device5}" "_nomac" ""
      expect -c "${eval_command_string}" &
    fi
    wait
    echo "Done eval nomac"
fi

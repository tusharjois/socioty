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

export_sh="/Users/logan/export-esp.sh"
device1="usbserial-144460"
device2="usbserial-1444710"
device3="usbserial-1444720"
device4="usbserial-1444730"
device5="usbserial-1444740"
runs=1000

# Running socioty_esp32_init sequence
if [[ $2 == "init" ]]; then
    for n in {5..12}
    do
        for (( t=$n-2; t<=$n; t++ ))
        do
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device1}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device1} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device2}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device2} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device3}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device3} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device4}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device4} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
            expect -c "
                set timeout 600
                log_user 0
                set logfile \"$1/init_${device5}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device5} --bin socioty_esp32_init --release\"
                expect \"Fin.\" {send \x03}" &
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
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device1}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device1} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device2}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device2} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device3}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device3} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device4}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device4} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
            expect -c "
                set timeout 3600
                log_user 0
                set logfile \"$1/reconstruct_${device5}_run_${n}_${t}.txt\"
                log_file -a \$logfile
                spawn bash -c \"source ${export_sh}; RUNS=${runs} CLIENTS=${n} K=${t} cargo espflash --monitor /dev/tty.${device5} --bin socioty_esp32_reconstruct --release\"
                expect \"Fin.\" {send \x03}" &
            wait
        done
    done
    echo "Done reconstruct"
fi
# Running socioty_esp32_eval sequence
if [[ $4 == "eval" ]]; then
    expect -c "
        set timeout 600
        log_user 0
        set logfile \"$1/eval_${device1}.txt\"
        log_file -a \$logfile
        spawn bash -c \"source ${export_sh}; RUNS=${runs} cargo espflash --monitor /dev/tty.${device1} --bin socioty_esp32_eval --features aead --release\"
        expect \"Fin.\" {send \x03}" &
    expect -c "
        set timeout 600
        log_user 0
        set logfile \"$1/eval_${device2}.txt\"
        log_file -a \$logfile
        spawn bash -c \"source ${export_sh}; RUNS=${runs} cargo espflash --monitor /dev/tty.${device2} --bin socioty_esp32_eval --features aead --release\"
        expect \"Fin.\" {send \x03}" &
    expect -c "
        set timeout 600
        log_user 0
        set logfile \"$1/eval_${device3}.txt\"
        log_file -a \$logfile
        spawn bash -c \"source ${export_sh}; RUNS=${runs} cargo espflash --monitor /dev/tty.${device3} --bin socioty_esp32_eval --features aead --release\"
        expect \"Fin.\" {send \x03}" &
    expect -c "
        set timeout 600
        log_user 0
        set logfile \"$1/eval_${device4}.txt\"
        log_file -a \$logfile
        spawn bash -c \"source ${export_sh}; RUNS=${runs} cargo espflash --monitor /dev/tty.${device4} --bin socioty_esp32_eval --features aead --release\"
        expect \"Fin.\" {send \x03}" &
    expect -c "
        set timeout 600
        log_user 0
        set logfile \"$1/eval_${device5}.txt\"
        log_file -a \$logfile
        spawn bash -c \"source ${export_sh}; RUNS=${runs} cargo espflash --monitor /dev/tty.${device5} --bin socioty_esp32_eval --features aead --release\"
        expect \"Fin.\" {send \x03}" &
    wait
    echo "Done eval"
fi

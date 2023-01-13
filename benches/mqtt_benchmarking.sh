#!/bin/bash

broker_ip="128.220.247.240"
nodes=("node0001" "node0002" "node0003" "node0004" "node0005" "node0006" "node0007" "node0008"
       "node0009" "node0010" "node0011" "node0012")
iterations=1000
stamp=1675014677

if [[ $1 == "configs" ]]; then
    for n in {5..12}
    do
        init_config_str="broker = \"${broker_ip}\"\n"
        init_config_str+="clients = [\n"
        for (( i=0; i<$n; i++ ))
        do
            init_config_str+="\"${nodes[i]}\",\n"
        done
        init_config_str+="]\n"
        for (( t=$n-2; t<=$n; t++ ))
        do
            echo "Creating configs for: n=${n} t=${t}"
            run_dir="run_${n}_${t}"
            mkdir ${run_dir}
            echo -e "${init_config_str}k=${t}" > "${run_dir}/init_config_${n}_${t}.toml"
            cd "${run_dir}/"
            #eval "../socioty_coap_init init_config_${n}_${t}"
    	    eval "/home/lkostic1/socioty-main/target/release/socioty_mqtt_init init_config_${n}_${t}.toml"
    	    for (( i=1; i<=$n; i++ ))
            do
                let j=i-1
                eval "scp ${nodes[j]}_config.toml node${i}:mqtt/${run_dir}_node.toml"
            done
            cd ..
        done
    done
fi

if [[ $2 == "run" ]]; then
    eval "rumqttd --config /home/lkostic1/rumqttd.toml > /dev/null 2>&1 &"
    for n in {5..12}
    do
        for (( t=$n-2; t<=$n; t++ )) 
        do
            echo "Running: n=${n} t=${t}"
            # Start the nodes
            echo "Starting nodes"
    	    for (( i=1; i<=$n; i++ ))
            do
                #eval "ssh node${i} 'nohup /home/logan/mqtt/socioty_mqtt_node mqtt/run_${n}_${t}_node.toml > mqtt/${n}_${t}.output 2>&1 &'"
                eval "ssh node${i} 'nohup /home/logan/mqtt/socioty_mqtt_node mqtt/run_${n}_${t}_node.toml > /dev/null 2>&1 &'"
            done
            sleep 3
            
            echo "Running iterations"
    	    touch run_${n}_${t}_output.txt
	        for (( itr=0; itr<$iterations; itr++ ))
	        do
    	        echo "Iteration${itr}: " >> run_${n}_${t}_output.txt
	    	    #eval "/home/lkostic1/socioty-main/target/release/socioty_mqtt_request run_${n}_${t}/request_config.toml $stamp $t"
	    	    eval "/home/lkostic1/socioty-main/target/release/socioty_mqtt_request run_${n}_${t}/request_config.toml $stamp $t" >> run_${n}_${t}_output.txt
            done

            echo "Killing procs now"
	        for (( i=1; i<=$n; i++ ))
            do
                eval "ssh node${i} './cleanup_procs socioty_mqtt_node'"
            done
            echo "done killing procs"
            sleep 3
        done
    done
    ps -ef | grep "rumqttd" | grep -v grep | awk '{ print $2 }' | xargs kill
fi


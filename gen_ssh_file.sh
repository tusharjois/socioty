#!/bin/bash

. /ssh_pi.configs

numPiTwoThree=${#pi_two_three[@]}
numPiZero=${#pi_zero[@]}

outputstring=""
for (( i=0; i<${numPiTwoThree}; i++ ));
do
  outputstring+="Host node$i\n" 
  outputstring+="\tHostName ${pi_two_three[$i]}\n" 
  outputstring+="\tUser node\n"
  outputstring+="\tIdentityFile ~/.ssh/socioty_nodes\n"
  outputstring+="\tIdentitiesOnly Yes\n"
  #echo "Host node$i" 
  #echo -e "\tHostName ${pi_two_three[$i]}" 
  #echo -e "\tUser node"
  #echo -e "\tIdentityFile ~/.ssh/socioty_nodes"
  #echo -e "\tIdentitiesOnly Yes"
done

for (( i=${numPiTwoThree}; i<(${numPiTwoThree}+${numPiZero}); i++ ));
do
  outputstring+="Host node$i\n" 
  outputstring+="\tHostName ${pi_zero[$i-${numPiTwoThree}]}\n" 
  outputstring+="\tUser node\n"
  outputstring+="\tIdentityFile ~/.ssh/socioty_nodes\n"
  outputstring+="\tIdentitiesOnly Yes\n"
done

echo -e "$outputstring" > ~/.ssh/config 



#!/bin/bash
for i in {0..127}; 
#for i in {0..0}; 
  do 
  let att1=$i*10
  #for j in {0..127}; 
  for j in {0..0};  
  do
    let att2=$att1
    #let att2=$j*1
    echo $att1 $att2
    ./calib.sh $1 $att1 $att2
  done
done

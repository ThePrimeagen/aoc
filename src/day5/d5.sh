#!/usr/bin/env bash

high=0
while IFS= read -r line; do echo "$((2#`echo $line | tr FBLR 0101`))"; done < input.test | sort -n | tail -1
val=$(while IFS= read -r line; do echo "$((2#`echo $line | tr FBLR 0101`))"; done < input.prod | sort -rn)

# how to do this?
low=13
high=978

echo $(($high * ($high + 1) / 2 - ($low - 1) * ($low) / 2 - `echo "${val[@]}" | paste -sd+ | bc`))

#for i in $val; do
#    if [ -z "$last" ]; then
#        last=$i
#        continue
#    fi
#
#    diff=$(($last - $i))
#    if [ $diff -ne 1 ]; then
#        echo "LOOK $last $i"
#    fi
#
#    last=$i
#done
#

#! /bin/bash

INPUT=$(cat input)

SUM=0

for MASS in $INPUT; do
  FUEL_FOR_MODULE=$(target/debug/day01 $MASS)
  SUM=$(expr $FUEL_FOR_MODULE + $SUM)
done

echo $SUM

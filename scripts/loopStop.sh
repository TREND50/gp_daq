#! /bin/sh


# Stop run on all antennas
cat boardsIn.txt | while read BOARDID
do
  echo "Now stopping board" $BOARDID
  ./stop.sh $BOARDID
done


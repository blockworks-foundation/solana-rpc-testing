#!/bin/bash

if [ -z "$1" ]
  then
      cat <<EOF

usage: $0 OUTPUT_DIR [TEST ARGS]
EOF
  exit 1
fi


for i in {5..200..5}
do
   cargo run --release -- -a -t $i -o "$1/$i.json" "${@:2}"
   sleep 10s
done

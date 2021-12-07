#!/usr/bin/bash

rc=0

for day in day*; do
    logs=$(cargo run --release --bin "$day" "$day/input.txt" 2> /dev/null)

    if [ "$logs" == "$(cat $day/solution.txt)" ]; then
        echo "$day: OK"
    else
        echo "$day: FAIL"
        rc=1
    fi
done

exit $rc

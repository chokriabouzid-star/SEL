#!/bin/bash

# Start mission
./target/release/sel-engine execute --mission test_mission.json &
PID=$!

# Kill it mid-execution
sleep 0.5
kill -9 $PID

# Check facts
echo "Facts after crash:"
cat facts_*.jsonl

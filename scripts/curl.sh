#!/bin/bash

MAC_ADDR="F4:93:9F:F4:04:5B"
BIND_ADDR="0.0"
BROADCAST_ADDR=""

curl -X POST http://127.0.0.1:8080/send_wol \
-H "Content-Type: application/json" \
-d '{"mac_address": "'$MAC_ADDR'", "bind_address": "'$BIND_ADDR'", "broadcast_address": "'$BROADCAST_ADDR'"}'
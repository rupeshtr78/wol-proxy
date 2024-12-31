#!/bin/bash

MAC_ADDR="F4:93:9F:F4:04:5B"
BIND_ADDR="0.0"
BROADCAST_ADDR=""

curl -X POST http://10.0.0.213:8090/wol \
-H "Content-Type: application/json" \
-d '{"mac_address": "'$MAC_ADDR'", "bind_address": "'$BIND_ADDR'", "broadcast_address": "'$BROADCAST_ADDR'"}' \
-b "wol-cookie=we12come.52fd317cfe9453f8a3fc425fabfd83fd6523a2489cf8d5ec9b73085fc19018ba"
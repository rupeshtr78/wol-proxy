#!/bin/bash

MAC_ADDR="F4:93:9F:F4:04:5B"
BIND_ADDR="0.0"
BROADCAST_ADDR=""

curl -X POST http://10.0.0.42:9080/wol \
-H "Content-Type: application/json" \
-d '{"mac_address": "'$MAC_ADDR'", "bind_address": "'$BIND_ADDR'", "broadcast_address": "'$BROADCAST_ADDR'"}' \
-b "wol-cookie=we12come.39a64a52139d4d95cd0f46982a04e4a0b40bd22930c780a0edbe3ee06c51096c"

#!/bin/bash

MAC_ADDR="F4:93:9F:F4:04:5B"
BIND_ADDR="0.0"
BROADCAST_ADDR=""

curl -X POST http://76.22.85.85:8644/wol \
-H "Content-Type: application/json" \
-d '{"mac_address": "'$MAC_ADDR'", "bind_address": "'$BIND_ADDR'", "broadcast_address": "'$BROADCAST_ADDR'"}' \
-b "wol-cookie=we12come.5e0ab8eb28416cf821fbe09777a0f902ad3f18ab35f479e44c2ef4c07f4cc3e4"

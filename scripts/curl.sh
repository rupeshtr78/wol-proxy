#!/bin/bash

MAC_ADDR="F4:93:9F:F4:04:5B"
BIND_ADDR="0.0"
BROADCAST_ADDR=""
COOKIE_NAME=wol-cookie
COOKIE=wol-cookie=wol-proxy.23664c370774284a7620cb511bef8892e49b844cc5e8725ed8e7af7e82719c34

if WOL_TLS == "false" then 
    curl -X POST http://10.0.0.42:9080/wol \
    -H "Content-Type: application/json" \
    -d '{"mac_address": "'$MAC_ADDR'", "bind_address": "'$BIND_ADDR'", "broadcast_address": "'$BROADCAST_ADDR'"}' \
    -b $COOKIE_NAME=$COOKIE
else
    curl --cert client.crt --key client.key -X post https://rupesh.com:9080/wol \
    -H "Content-Type: application/json" \
    -d '{"mac_address": "'$MAC_ADDR'", "bind_address": "'$BIND_ADDR'", "broadcast_address": "'$BROADCAST_ADDR'"}' \
    -b $COOKIE_NAME=$COOKIE


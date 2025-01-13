#!/bin/bash

MAC_ADDR="F4:93:9F:F4:04:5B"
BIND_ADDR="0.0"
BROADCAST_ADDR=""

# Ensure required environment variables are set
if [[  -z "$COOKIE_NAME" || -z "$COOKIE" ]]; then
    echo "Error: Missing required environment variables."
    exit 1
fi

# Define the base URL and request payload
BASE_URL="http://rupesh.com:9080/wol"
PAYLOAD=$(cat <<EOF
{
    "mac_address": "$MAC_ADDR",
    "bind_address": "$BIND_ADDR",
    "broadcast_address": "$BROADCAST_ADDR"
}
EOF
)

# Set the appropriate curl command based on TLS configuration
if [[ "$WOL_TLS" == "false" ]]; then
    curl -X POST "$BASE_URL" \
        -H "Content-Type: application/json" \
        -d "$PAYLOAD" \
        -b "$COOKIE_NAME=$COOKIE"
else
    if [[ ! -f "client.crt" || ! -f "client.key" ]]; then
        echo "Error: TLS client certificate or key file not found."
        exit 1
    fi
    curl --cert client.crt --key client.key -X POST "https://rupesh.forsynet.com:9080/wol" \
        -H "Content-Type: application/json" \
        -d "$PAYLOAD" \
        -b "$COOKIE_NAME=$COOKIE"
fi

# Check the exit status of the curl command
if [[ $? -ne 0 ]]; then
    echo "Error: curl request failed."
    exit 1
fi
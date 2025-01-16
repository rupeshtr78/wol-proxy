#!/bin/bash

curl -X POST http://localhost:8098/status \
     -H "Content-Type: application/json" \
     -d '{"ip": "10.0.0.42", "port": "9080"}' \
     -b "$COOKIE_NAME=$COOKIE"
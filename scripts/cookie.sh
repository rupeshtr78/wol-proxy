#!/bin/bash

COOKIE_NAME="wol-cookie"
# Generate the secret key
COOKIE_SECRET_KEY=$(openssl rand -base64 32) # 32 bytes = 256 bits

# Use the secret key to sign the cookie value using HMAC
cookie_secret_value="we12come"
signature=$(echo -n "$cookie_secret_value" | openssl dgst -sha256 -hmac "$COOKIE_SECRET_KEY" | awk '{print $2}')
signed_cookie_value="$cookie_secret_value.$signature" # Combine value and signature
cookie_string="$COOKIE_NAME=$signed_cookie_value"

# Output the secret key and signed cookie
echo "export COOKIE_SECRET_KEY=$COOKIE_SECRET_KEY"
echo "export COOKIE_NAME=$COOKIE_NAME"
echo "export COOKIE_SECRET_VALUE=$cookie_secret_value"
echo "export COOKIE=$cookie_string"

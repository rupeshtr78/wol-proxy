#!/bin/bash

COOKIE_NAME="wol-cookie"
# Generate the secret key
COOKIE_SECRET_KEY=$(openssl rand -base64 32) # 32 bytes = 256 bits

# Use the secret key to sign the cookie value using HMAC
cookie_secret_value="wol-proxy" # changeme
signature=$(echo -n "$cookie_secret_value" | openssl dgst -sha256 -hmac "$COOKIE_SECRET_KEY" | awk '{print $2}')
signed_cookie_value="$cookie_secret_value.$signature" # Combine value and signature
cookie_string="$COOKIE_NAME=$signed_cookie_value"

# Output the secret key and signed cookie
echo "export COOKIE_SECRET_KEY=$COOKIE_SECRET_KEY"
echo "export COOKIE_NAME=$COOKIE_NAME"
echo "export COOKIE_SECRET_VALUE=$cookie_secret_value"
echo "export COOKIE=$cookie_string"
echo "export WOL_PORT=8098"

# write to .env file
echo "COOKIE_SECRET_KEY=$COOKIE_SECRET_KEY" > .env
echo "COOKIE_NAME=$COOKIE_NAME" >> .env
echo "COOKIE_SECRET_VALUE=$cookie_secret_value" >> .env
echo "COOKIE=$cookie_string" >> .env
echo "WOL_PORT=8098" >> .env
echo "WOL_SERVER_CERT=certs/server.crt" >> .env
echo "WOL_SERVER_KEY=certs/server.key" >> .env
echo "WOL_TLS=true" >> .env
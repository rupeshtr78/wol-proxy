#!/bin/bash

# Create wol-proxy service
# replace the path with the actual path of the binary file
# replace the user with the actual user
# replace environment variables with the actual values
sudo scp scripts/wol-proxy.service pi@10.xx.xx.xx:/etc/systemd/system/wol-proxy.service

# Reload systemd manager configuration
sudo systemctl daemon-reload
sudo systemctl enable wol-proxy.service
sudo systemctl start wol-proxy.service
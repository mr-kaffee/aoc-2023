#!/bin/bash

git config --global --add safe.directory /workspace

# start web server
sudo nginx

# Execute the main command (e.g., your Rust application)
exec "$@"

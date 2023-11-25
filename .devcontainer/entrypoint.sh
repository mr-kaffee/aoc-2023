#!/bin/bash

# start web server
sudo nginx

# Execute the main command (e.g., your Rust application)
exec "$@"

#!/bin/bash

git config --global --add safe.directory /workspace

# Execute the main command (e.g., your Rust application)
exec "$@"

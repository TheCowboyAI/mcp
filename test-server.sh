#!/usr/bin/env bash

# Create named pipes for communication
PIPE_IN="/tmp/mcp-server-in"
PIPE_OUT="/tmp/mcp-server-out"

# Clean up existing pipes
rm -f "$PIPE_IN" "$PIPE_OUT"

# Create new pipes
mkfifo "$PIPE_IN"
mkfifo "$PIPE_OUT"

# Start server in background
RUST_LOG=debug nix run < "$PIPE_IN" > "$PIPE_OUT" &
SERVER_PID=$!

# Function to send a message and wait for response
send_message() {
    echo "$1" > "$PIPE_IN"
    read -r RESPONSE < "$PIPE_OUT"
    echo "$RESPONSE"
}

# Initialize server
echo "Initializing server..."
send_message '{"jsonrpc": "2.0", "id": "1", "method": "initialize", "params": {"capabilities": {}, "implementation": {"name": "nix-inspector-mcp", "version": "0.2.0"}}}'

# Get system info
echo -e "\nGetting system info..."
send_message '{"jsonrpc": "2.0", "id": "2", "method": "get_system_info", "params": {}}'

# Clean up
kill $SERVER_PID
rm -f "$PIPE_IN" "$PIPE_OUT" 